-- Profiles: one row per auth user, auto-created on signup.
create table if not exists public.profiles (
  user_id      uuid primary key references auth.users (id) on delete cascade,
  display_name text,
  created_at   timestamptz not null default now()
);

-- Saves: one serialized game blob per save, owned by a user.
create table if not exists public.saves (
  id         uuid primary key default gen_random_uuid(),
  user_id    uuid not null references auth.users (id) on delete cascade,
  name       text not null default 'Career',
  manager    text,                         -- denormalized for listing without deserializing
  data       bytea not null,               -- serialized Game blob
  created_at timestamptz not null default now(),
  updated_at timestamptz not null default now()
);

create index if not exists saves_user_id_idx on public.saves (user_id);

-- Row Level Security: users can only touch their own rows.
alter table public.profiles enable row level security;
alter table public.saves    enable row level security;

create policy "profiles_select_own" on public.profiles
  for select using (auth.uid() = user_id);
create policy "profiles_insert_own" on public.profiles
  for insert with check (auth.uid() = user_id);
create policy "profiles_update_own" on public.profiles
  for update using (auth.uid() = user_id);

create policy "saves_select_own" on public.saves
  for select using (auth.uid() = user_id);
create policy "saves_insert_own" on public.saves
  for insert with check (auth.uid() = user_id);
create policy "saves_update_own" on public.saves
  for update using (auth.uid() = user_id);
create policy "saves_delete_own" on public.saves
  for delete using (auth.uid() = user_id);

-- Auto-create a profile row when a new auth user signs up.
create or replace function public.handle_new_user()
returns trigger
language plpgsql
security definer set search_path = ''
as $$
begin
  insert into public.profiles (user_id, display_name)
  values (new.id, coalesce(new.raw_user_meta_data ->> 'display_name', split_part(new.email, '@', 1)));
  return new;
end;
$$;

drop trigger if exists on_auth_user_created on auth.users;
create trigger on_auth_user_created
  after insert on auth.users
  for each row execute function public.handle_new_user();

-- Keep saves.updated_at fresh on update.
create or replace function public.touch_updated_at()
returns trigger
language plpgsql
as $$
begin
  new.updated_at = now();
  return new;
end;
$$;

drop trigger if exists saves_touch_updated_at on public.saves;
create trigger saves_touch_updated_at
  before update on public.saves
  for each row execute function public.touch_updated_at();
