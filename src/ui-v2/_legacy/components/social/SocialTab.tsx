import { useEffect, useMemo, useRef, useState } from "react";
import { BadgeCheck, Heart, MessageCircle, Repeat2, Sparkles } from "lucide-react";
import { useTranslation } from "react-i18next";
import type { GameStateData, PlayerData, SocialPostData, TeamData } from "@/store/gameStore";
import { Badge } from "@/ui-v2/_legacy/components/ui";
import { formatDateShort } from "@/lib/common/helpers";
import { assetUrl } from "@/lib/assetUrl";
import { resolveSocialAvatar } from "@/lib/social/resolveSocialAvatar";
import { createManagerSocialPost, relocalizeSocialFeed } from "@/services/socialService";
interface SocialTabProps {
  gameState: GameStateData;
  onGameUpdate: (state: GameStateData) => void;
}

const AUTHOR_LABELS: Record<string, string> = {
  Team: "Team",
  Player: "Player",
  Fan: "Fan",
  Analyst: "Analyst",
  Journalist: "Media",
  MemeAccount: "Meme",
  Manager: "Manager",
};

const SENTIMENT_VARIANT: Record<string, "primary" | "accent" | "success" | "danger" | "neutral"> = {
  Hype: "success",
  Calm: "neutral",
  Worried: "accent",
  Angry: "danger",
  Meltdown: "danger",
  Copium: "primary",
};

function displayAuthorName(post: SocialPostData): string {
  return post.author_name;
}

function displayAuthorHandle(post: SocialPostData): string {
  return post.author_handle;
}

function formatCount(value: number): string {
  if (value >= 1_000_000) return `${(value / 1_000_000).toFixed(1)}M`;
  if (value >= 1_000) return `${(value / 1_000).toFixed(1)}K`;
  return String(value);
}

function authorRing(post: SocialPostData): string {
  switch (post.author_type) {
    case "Team":
      return "from-primary to-primary/70";
    case "Player":
      return "from-emerald-500 to-emerald-400";
    case "Analyst":
    case "Journalist":
      return "from-indigo-500 to-indigo-400";
    case "MemeAccount":
      return "from-pink-500 to-orange-400";
    default:
      return "from-muted to-muted-foreground/30";
  }
}

function verifiedMeta(post: SocialPostData): { color: string; title: string } | null {
  if (post.author_type === "Team") {
    return { color: "text-amber-400", title: "Golden verified team" };
  }
  if (post.author_type === "Player") {
    return { color: "text-sky-500", title: "Verified player" };
  }
  return null;
}

function Avatar({
  post,
  teams,
  players,
  accounts,
}: {
  post: SocialPostData;
  teams: TeamData[];
  players: PlayerData[];
  accounts: GameStateData["social_accounts"];
}) {
  const rawSrc = resolveSocialAvatar(
    post,
    accounts ?? [],
    teams,
    players,
  );
  const src = assetUrl(rawSrc);

  return (
    <div className={`relative flex h-12 w-12 shrink-0 items-center justify-center overflow-hidden rounded-full bg-linear-to-br ${authorRing(post)} font-heading text-sm font-bold text-white`}>
      {src ? (
        <img
          src={src}
          alt={post.author_name}
          className="absolute inset-0 h-full w-full object-cover"
          loading="lazy"
          onError={(event) => {
            event.currentTarget.style.display = "none";
          }}
        />
      ) : (
        <span>{displayAuthorName(post).slice(0, 2).toUpperCase()}</span>
      )}
    </div>
  );
}

export default function SocialTab({ gameState, onGameUpdate }: SocialTabProps) {
  const { t, i18n } = useTranslation();
  const [likedPostIds, setLikedPostIds] = useState<Set<string>>(() => new Set());
  const [repostedPostIds, setRepostedPostIds] = useState<Set<string>>(() => new Set());
  const [composerText, setComposerText] = useState("");
  const [posting, setPosting] = useState(false);
  const [postError, setPostError] = useState<string | null>(null);
  const lastLocalizedLanguageRef = useRef<string>("");
  const posts = useMemo(
    () =>
      [...(gameState.social_posts ?? [])]
        .filter((post) => {
          if (post.id.endsWith("_fan_bouzys_fnatic")) {
            return i18n.language.toLowerCase().startsWith("es");
          }
          return true;
        })
        .sort((a, b) => b.date.localeCompare(a.date) || b.id.localeCompare(a.id)),
    [gameState.social_posts, i18n.language],
  );

  useEffect(() => {
    const language = i18n.language;
    if (!language || lastLocalizedLanguageRef.current === language) {
      return;
    }
    lastLocalizedLanguageRef.current = language;
    relocalizeSocialFeed(language)
      .then((updatedGameState) => {
        onGameUpdate(updatedGameState);
      })
      .catch(() => {
        // no-op: keep current timeline if relocalization fails
      });
  }, [i18n.language, onGameUpdate]);

  function togglePostId(postIds: Set<string>, postId: string): Set<string> {
    const nextPostIds = new Set(postIds);
    if (nextPostIds.has(postId)) {
      nextPostIds.delete(postId);
    } else {
      nextPostIds.add(postId);
    }
    return nextPostIds;
  }

  async function handlePublishPost(): Promise<void> {
    const text = composerText.trim();
    if (!text) {
      return;
    }

    setPosting(true);
    setPostError(null);
    try {
      const updatedGameState = await createManagerSocialPost(text);
      onGameUpdate(updatedGameState);
      setComposerText("");
    } catch (error) {
      setPostError(error instanceof Error ? error.message : "No se pudo publicar el post.");
    } finally {
      setPosting(false);
    }
  }

  if (posts.length === 0) {
    return (
      <div className="mx-auto flex max-w-3xl flex-col items-center justify-center py-16 text-center">
        <div className="mb-4 flex h-14 w-14 items-center justify-center rounded-2xl bg-primary/10 text-primary">
          <Sparkles className="h-7 w-7" />
        </div>
        <h2 className="font-heading text-xl font-bold uppercase tracking-wider text-foreground text-foreground">
          {t("social.emptyTitle", { defaultValue: "The timeline is quiet" })}
        </h2>
        <p className="mt-2 max-w-md text-sm text-muted-foreground dark:text-muted-foreground/70">
          {t("social.emptyBody", {
            defaultValue: "Play matches and the community will start posting banter, hot takes, and questionable analysis.",
          })}
        </p>
      </div>
    );
  }

  return (
    <div className="mx-auto max-w-2xl overflow-hidden rounded-2xl border border-border bg-card shadow-sm border-border bg-card">
      <div className="sticky top-0 z-10 border-b border-border bg-card/90 px-5 py-4 backdrop-blur border-border bg-card/90">
        <h2 className="font-heading text-xl font-bold text-foreground text-foreground">
          {t("social.title", { defaultValue: "Social" })}
        </h2>
        <p className="text-sm text-muted-foreground dark:text-muted-foreground/70">
          {t("social.subtitle", { defaultValue: "Community timeline" })}
        </p>

        <div className="mt-3 rounded-xl border border-border bg-card p-3 border-border bg-muted/50">
          <textarea
            value={composerText}
            maxLength={280}
            onChange={(event) => setComposerText(event.target.value)}
            placeholder={t("social.composerPlaceholder", { defaultValue: "¿Qué está pensando el míster hoy?" })}
            className="min-h-[84px] w-full resize-none bg-transparent text-sm text-foreground outline-none placeholder:text-muted-foreground/70 text-foreground"
          />
          <div className="mt-2 flex items-center justify-between">
            <span className="text-xs text-muted-foreground/70">{composerText.length}/280</span>
            <button
              type="button"
              disabled={posting || composerText.trim().length === 0}
              onClick={() => {
                void handlePublishPost();
              }}
              className="rounded-full bg-primary px-4 py-1.5 text-xs font-bold uppercase tracking-wider text-white transition hover:bg-primary/90 disabled:cursor-not-allowed disabled:opacity-50"
            >
              {posting
                ? t("social.posting", { defaultValue: "Publicando..." })
                : t("social.post", { defaultValue: "Postear" })}
            </button>
          </div>
          {postError ? <p className="mt-2 text-xs text-red-500">{postError}</p> : null}
        </div>
      </div>

      {posts.map((post) => (
        <article key={post.id} className="flex gap-3 border-b border-border px-5 py-4 transition-colors hover:bg-muted border-border hover:bg-muted/50">
          {(() => {
            const liked = likedPostIds.has(post.id);
            const reposted = repostedPostIds.has(post.id);
            const likes = post.likes + (liked ? 1 : 0);
            const reposts = post.reposts + (reposted ? 1 : 0);

            return (
              <>
          <Avatar
            post={post}
            teams={gameState.teams}
            players={gameState.players}
            accounts={gameState.social_accounts}
          />

          <div className="min-w-0 flex-1">
            <div className="flex min-w-0 flex-wrap items-center gap-x-1.5 gap-y-1">
              <span className="truncate font-bold text-foreground text-foreground">
                {displayAuthorName(post)}
              </span>
              {verifiedMeta(post) ? (
                <BadgeCheck className={`h-4 w-4 shrink-0 ${verifiedMeta(post)?.color}`} aria-label={verifiedMeta(post)?.title} />
              ) : null}
              <span className="truncate text-sm text-muted-foreground dark:text-muted-foreground/70">{displayAuthorHandle(post)}</span>
              <span className="text-sm text-muted-foreground/70">·</span>
              <span className="text-sm text-muted-foreground dark:text-muted-foreground/70">
                {formatDateShort(post.date, i18n.language)}
              </span>
            </div>

            <p className="mt-1 whitespace-pre-line text-sm leading-normal text-foreground text-foreground">
              {post.body}
            </p>

            {post.media_url ? (
              <div className="mt-3 overflow-hidden rounded-xl border border-border border-border">
                <img
                  src={post.media_url}
                  alt={t("social.mediaAlt", { defaultValue: "Post media" })}
                  className="max-h-96 w-full object-cover"
                  loading="lazy"
                />
              </div>
            ) : null}

            <div className="mt-3 flex flex-wrap gap-2">
              <Badge variant="neutral">{AUTHOR_LABELS[post.author_type] ?? post.author_type}</Badge>
              <Badge variant={SENTIMENT_VARIANT[post.sentiment] ?? "neutral"}>{post.sentiment}</Badge>
            </div>

            <div className="mt-4 flex max-w-md items-center justify-between text-sm text-muted-foreground dark:text-muted-foreground/70">
              <span className="inline-flex items-center gap-1.5 transition-colors hover:text-sky-500">
                <MessageCircle className="h-4 w-4" /> {formatCount(post.replies)}
              </span>
              <button
                type="button"
                aria-pressed={reposted}
                onClick={() => setRepostedPostIds((currentPostIds) => togglePostId(currentPostIds, post.id))}
                className={`inline-flex items-center gap-1.5 transition-colors hover:text-emerald-500 ${reposted ? "text-emerald-500" : ""}`}
              >
                <Repeat2 className="h-4 w-4" /> {formatCount(reposts)}
              </button>
              <button
                type="button"
                aria-pressed={liked}
                onClick={() => setLikedPostIds((currentPostIds) => togglePostId(currentPostIds, post.id))}
                className={`inline-flex items-center gap-1.5 transition-colors hover:text-pink-500 ${liked ? "text-pink-500" : ""}`}
              >
                <Heart className={`h-4 w-4 ${liked ? "fill-current" : ""}`} /> {formatCount(likes)}
              </button>
            </div>
          </div>
              </>
            );
          })()}
        </article>
      ))}
    </div>
  );
}



