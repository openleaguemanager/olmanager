-- Pin search_path on the updated_at trigger fn.
create or replace function public.touch_updated_at()
returns trigger
language plpgsql
security invoker
set search_path = ''
as $$
begin
  new.updated_at = now();
  return new;
end;
$$;

-- handle_new_user must stay SECURITY DEFINER (it writes profiles on signup),
-- but it should only ever run from the auth.users trigger, never as a public
-- RPC. Revoke direct EXECUTE from API roles. The trigger still works because
-- triggers run as the table owner, independent of these grants.
revoke execute on function public.handle_new_user() from public, anon, authenticated;
