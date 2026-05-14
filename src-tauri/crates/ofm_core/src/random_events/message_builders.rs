use super::{action, format_money, params};
use crate::narrative::{NarrativeSelector, load_default_content_pack};
use domain::{message::*};
use domain::manager::Manager;
use domain::team::Team;
use domain::player::Player;
use chrono::{Datelike, NaiveDate, Weekday};
use rand::RngExt;
use rand::distr::Distribution;
use rand::distr::weighted::WeightedIndex;
use rand::seq::SliceRandom;

// ---------------------------------------------------------------------------
// Message builders
// ---------------------------------------------------------------------------

pub(super) fn sponsor_offer_message(
    msg_id: &str,
    team_name: &str,
    sponsor: &str,
    amount: u64,
    date: &str,
) -> InboxMessage {
    InboxMessage::new(
        msg_id.to_string(),
        format!("Sponsorship Offer — {}", sponsor),
        format!(
            "Good news, boss! {} has expressed interest in becoming a sponsor of {}.\n\n\
            They're offering a weekly payment of €{} over the next 12 weeks in exchange for advertising space at the training ground.\n\n\
            This seems like a reasonable deal, but it's your call.",
            sponsor, team_name, format_money(amount)
        ),
        "Commercial Director".to_string(),
        date.to_string(),
    )
    .with_category(MessageCategory::Finance)
    .with_priority(MessagePriority::Normal)
    .with_sender_role("Commercial Director")
    .with_action(action(
        "respond", "Respond", "be.msg.event.respond",
        ActionType::ChooseOption {
            options: vec![
                ActionOption {
                    id: "accept".to_string(),
                    label: "Accept the deal".to_string(),
                    description: format!("Receive €{} in sponsorship income.", format_money(amount)),
                    label_key: Some("be.msg.sponsor.options.accept.label".to_string()),
                    description_key: Some("be.msg.sponsor.options.accept.description".to_string()),
                },
                ActionOption {
                    id: "decline".to_string(),
                    label: "Decline politely".to_string(),
                    description: "Turn down the offer. No financial impact.".to_string(),
                    label_key: Some("be.msg.sponsor.options.decline.label".to_string()),
                    description_key: Some("be.msg.sponsor.options.decline.description".to_string()),
                },
            ],
        },
    ))
    .with_i18n(
        "be.msg.sponsor.subject",
        "be.msg.sponsor.body",
        params(&[("sponsor", sponsor), ("team", team_name), ("amount", &format_money(amount))]),
    )
    .with_sender_i18n("be.sender.commercialDirector", "be.role.commercialDirector")
}

pub(super) fn training_injury_message(
    msg_id: &str,
    player_id: &str,
    player_name: &str,
    injury_name: &str,
    days: u32,
    date: &str,
) -> InboxMessage {
    let mut rng = rand::rng();
    let variations = [
        format!(
            "Bad news from the training ground. {} has picked up a {} during today's session.\n\n\
            The medical team estimates {} days on the sidelines. We'll monitor the recovery closely.",
            player_name,
            injury_name.to_lowercase(),
            days
        ),
        format!(
            "Unfortunately, {} went down in training today with a {}.\n\n\
            Initial assessment: out for approximately {} days. We'll keep you updated on their progress.",
            player_name,
            injury_name.to_lowercase(),
            days
        ),
    ];
    let idx = rng.random_range(0..variations.len());

    InboxMessage::new(
        msg_id.to_string(),
        format!("Injury — {} ({})", player_name, injury_name),
        variations[idx].clone(),
        "Head Physio".to_string(),
        date.to_string(),
    )
    .with_category(MessageCategory::Injury)
    .with_priority(MessagePriority::High)
    .with_sender_role("Head Physio")
    .with_action(action(
        "ack",
        "Understood",
        "be.msg.event.ack",
        ActionType::Acknowledge,
    ))
    .with_context(MessageContext {
        player_id: Some(player_id.to_string()),
        ..Default::default()
    })
    .with_i18n(
        "be.msg.trainingInjury.subject",
        &format!("be.msg.trainingInjury.body{}", idx),
        params(&[
            ("player", player_name),
            ("injury", injury_name),
            ("days", &days.to_string()),
        ]),
    )
    .with_sender_i18n("be.sender.headPhysio", "be.role.headPhysio")
}

pub(super) fn media_story_message(
    msg_id: &str,
    is_positive: bool,
    centered_on_player: bool,
    team: &Team,
    manager: &Manager,
    player: &Player,
    date: &str,
) -> Option<InboxMessage> {
    // I (Vincent LAMBERT) estimated that using this build_media_story_from_narrative function was not adapted to have 
    // a fine control over each media, especially considering that streamers can be affiliated with teams, speak a 
    // certain language that is not necessarily the one of the manager and/or the targetted player and can co-cast
    // multiple specific leagues. I anticipated that with the unique weight system associated with the 
    // NarrativeSelector, this would not be adapted. However, I think that this system is particularly adapted for
    // traditionnal media narratives (sheep-esports, dexerto, etc...). As this was not the focus of the 
    // feat(media): add more life and diversity in alternative media coverage #195 , I let it here without correcting
    // the issues with the build_media_story_from_narrative function.
    if let Some(message) = build_media_story_from_narrative(
        msg_id,
        &team.name,
        &player.id,
        &player.match_name,
        is_positive,
        date,
    ) {
        return Some(message);
    }

    let mut media_story: Option<InboxMessage>; 
    if centered_on_player {
        // The story can have an impact on the player mostly if it is in their language
        media_story = language_media_story(&msg_id,
                    is_positive,
                    centered_on_player,
                    team,
                    &player.nationality,
                    player,
                    date);
    } else {
        // Otherwise, the story is relevant only because it is centered on the team
        // in general and discovered because expressed in the manager language
        media_story = language_media_story(&msg_id,
                    is_positive,
                    centered_on_player,
                    team,
                    &manager.nationality,
                    player,
                    date);
    }
    // If there is not any story in the player/manager language, check for english stories
    if !media_story.is_some() {
        media_story = english_media_story(&msg_id,
                is_positive,
                centered_on_player,
                team,
                player,
                date);
    }
    media_story
}

pub fn build_media_story_from_narrative(
    _msg_id: &str,
    _team_name: &str,
    _player_id: &str,
    _player_name: &str,
    is_positive: bool,
    _date: &str,
) -> Option<InboxMessage> {
    let pack = load_default_content_pack().ok()?;
    let selector = NarrativeSelector::new(&pack);
    let tags = if is_positive {
        vec!["media", "positive"]
    } else {
        vec!["media", "pressure"]
    };
    let tones = if is_positive {
        vec!["professional", "analytical", "community"]
    } else {
        vec!["spicy", "pressure"]
    };
    let _template: &crate::narrative::SocialEventTemplate = selector.select_event(Some("default"), &tags, &tones)?;
    // Template was not used and this fonction was obviously work in progress for now so I commented it to make it clear.

    // let mut rng = rand::rng();
    // let variant = rng.random_range(0..2usize);

    // let (subject, body, subject_key, priority) = if is_positive {
    //     match variant {
    //         0 => (
    //             format!(
    //                 "Esportmaníacos praises {} — the desk is unanimous",
    //                 player_name
    //             ),
    //             format!(
    //                 "The Esportmaníacos panel spent a good chunk of today's stream on {} at {}.\n\n\
    //                 \"Nothing to criticize this week. The guy is absolutely dominating.\" \
    //                 The positive coverage should give confidence a boost in the locker room.",
    //                 player_name, team_name
    //             ),
    //             "be.msg.esportmaniacos.positive.subject0",
    //             MessagePriority::Low,
    //         ),
    //         _ => (
    //             format!(
    //                 "Esportmaníacos: {} is carrying {} this split",
    //                 player_name, team_name
    //             ),
    //             format!(
    //                 "The Esportmaníacos tertulianos rarely agree on anything, but {} got a full pass today: \
    //                 \"Consistent, solid, no dips in form. One of the best players in the league right now. \
    //                 {} should do everything to keep him\".\n\n\
    //                 Good for morale — and for the market value.",
    //                 player_name, team_name
    //             ),
    //             "be.msg.esportmaniacos.positive.subject1",
    //             MessagePriority::Low,
    //         ),
    //     }
    // } else {
    //     match variant {
    //         0 => (
    //             format!(
    //                 "Esportmaníacos goes after {} — panel shows no mercy",
    //                 player_name
    //             ),
    //             format!(
    //                 "The Esportmaníacos panel did not hold back today on {}: \
    //                 \"The guy has completely disappeared. What happened to him this split? \
    //                 {} can't keep relying on someone performing like this\".\n\n\
    //                 This kind of coverage tends to hit morale hard. Worth having a word with the player.",
    //                 player_name, team_name
    //             ),
    //             "be.msg.esportmaniacos.negative.subject0",
    //             MessagePriority::Normal,
    //         ),
    //         _ => (
    //             format!(
    //                 "Esportmaníacos questions {}'s consistency at {}",
    //                 player_name, team_name
    //             ),
    //             format!(
    //                 "Today's Esportmaníacos tertulianos session turned into a full breakdown of {}'s recent form. \
    //                 The verdict: \"Inconsistent. No regularity. Some days at top level, others completely invisible. \
    //                 {} deserves better output from someone in that role\".\n\n\
    //                 Keep an eye on the player's morale.",
    //                 player_name, team_name
    //             ),
    //             "be.msg.esportmaniacos.negative.subject1",
    //             MessagePriority::Normal,
    //         ),
    //     }
    // };

    // let body_key = if is_positive {
    //     format!("be.msg.esportmaniacos.positive.body{}", variant)
    // } else {
    //     format!("be.msg.esportmaniacos.negative.body{}", variant)
    // };

    // Some(
    //     InboxMessage::new(
    //         msg_id.to_string(),
    //         subject,
    //         body,
    //         "Esportmaníacos".to_string(),
    //         date.to_string(),
    //     )
    //     .with_category(MessageCategory::Media)
    //     .with_priority(priority)
    //     .with_sender_role("Panel")
    //     .with_action(action(
    //         "ack",
    //         "Noted",
    //         "be.msg.event.ack",
    //         ActionType::Acknowledge,
    //     ))
    //     .with_context(MessageContext {
    //         player_id: Some(player_id.to_string()),
    //         ..Default::default()
    //     })
    //     .with_i18n(
    //         subject_key,
    //         &body_key,
    //         params(&[
    //             ("player", player_name),
    //             ("team", team_name),
    //             ("effectId", &template.effect_id),
    //         ]),
    //     )
    //     .with_sender_i18n("be.sender.esportmaniacos", "be.role.esportmaniacos"),
    // )
    None
}

pub(super) fn international_callup_message(
    msg_id: &str,
    player_name: &str,
    nationality: &str,
    date: &str,
) -> InboxMessage {
    InboxMessage::new(
        msg_id.to_string(),
        format!("International Call-Up — {}", player_name),
        format!(
            "{} has been called up to the {} national team for an upcoming international window.\n\n\
            This is a great honor for the player and reflects well on the club. \
            They'll be in good spirits when they return, though keep an eye on their fatigue levels.",
            player_name, nationality
        ),
        "International Liaison".to_string(),
        date.to_string(),
    )
    .with_category(MessageCategory::LeagueInfo)
    .with_priority(MessagePriority::Normal)
    .with_sender_role("International Liaison")
    .with_action(action("ack", "Acknowledged", "be.msg.event.ack", ActionType::Acknowledge))
    .with_i18n(
        "be.msg.intlCallup.subject",
        "be.msg.intlCallup.body",
        params(&[("player", player_name), ("nationality", nationality)]),
    )
    .with_sender_i18n("be.sender.intlLiaison", "be.role.intlLiaison")
}

pub(super) fn community_event_message(msg_id: &str, team_name: &str, date: &str) -> InboxMessage {
    let mut rng = rand::rng();
    let events = [
        (
            "Community Open Day",
            format!(
                "{} hosted a community open day at the training ground today.\n\n\
                Fans got to meet the players and watch a training session. \
                The atmosphere was fantastic and it's done wonders for team spirit.",
                team_name
            ),
        ),
        (
            "Youth Coaching Session",
            format!(
                "Several first-team players from {} volunteered for a youth coaching session at a local school.\n\n\
                Great PR for the club, and the players seem energized by the experience.",
                team_name
            ),
        ),
        (
            "Charity Match Announcement",
            format!(
                "The club has organized a charity initiative in partnership with a local foundation.\n\n\
                {} continues to build strong ties with the community. The board is pleased with the positive image.",
                team_name
            ),
        ),
    ];
    let idx = rng.random_range(0..events.len());
    let (subject, body) = &events[idx];

    InboxMessage::new(
        msg_id.to_string(),
        subject.to_string(),
        body.clone(),
        "Community Manager".to_string(),
        date.to_string(),
    )
    .with_category(MessageCategory::System)
    .with_priority(MessagePriority::Low)
    .with_sender_role("Community Manager")
    .with_action(action(
        "ack",
        "Great",
        "be.msg.event.ack",
        ActionType::Acknowledge,
    ))
    .with_i18n(
        &format!("be.msg.community.subject{}", idx),
        &format!("be.msg.community.body{}", idx),
        params(&[("team", team_name)]),
    )
    .with_sender_i18n("be.sender.communityManager", "be.role.communityManager")
}

fn language_media_story(
    msg_id: &str,
    is_positive: bool,
    centered_on_player: bool,
    team: &Team,
    nationality: &str,
    player: &Player,
    date: &str,
) -> Option<InboxMessage> {
    // For now, only french, spanish and english medias (outside of streamers) have been covered.
    // English is the default for every language that does not have a media ecosystem (to our knowledge)
    match nationality {
        "DE" => english_media_story(msg_id, is_positive, centered_on_player, team, player, date), // No specific media known
        "EN" => english_media_story(msg_id, is_positive, centered_on_player, team, player, date),
        "ES" => spanish_media_story(msg_id, is_positive, centered_on_player, team, player, date),
        "FR" => french_media_story(msg_id, is_positive, centered_on_player, team, player, date),
        "PT" => english_media_story(msg_id, is_positive, centered_on_player, team, player, date), // No specific media known
        "BR" => english_media_story(msg_id, is_positive, centered_on_player, team, player, date), // No specific media known
        "IT" => english_media_story(msg_id, is_positive, centered_on_player, team, player, date), // No specific media known
        _ => english_media_story(msg_id, is_positive, centered_on_player, team, player, date),
    }
}

fn english_media_story(
    msg_id: &str,
    is_positive: bool,
    centered_on_player: bool,
    team: &Team,
    player: &Player,
    date: &str,
) -> Option<InboxMessage> {
    let mut rng = rand::rng();
    let mut messages = Vec::new(); 

    // Media stories can be podcasts broacasted on specific days
    if let Ok(today) = NaiveDate::parse_from_str(&date, "%Y-%m-%d") {
        if today.weekday() == Weekday::Wed {
            // HopEUm podcast
            messages.push(Some(hopeum_podcast_media_story(
                msg_id,
                is_positive,
                centered_on_player,
                team,
                player,
                date,
            )));
            // MindTheGap podcast
            messages.push(Some(mind_the_gap_podcast_media_story(
                msg_id,
                is_positive,
                centered_on_player,
                team,
                player,
                date,
            )));
        }
        if today.weekday() == Weekday::Fri {
            messages.push(Some(lec_podcast_media_story(
                msg_id,
                is_positive,
                centered_on_player,
                team,
                player,
                date,
            )));
        }
    }
    // Shuffle the messages and keep only one (ingored if empty)
    messages.shuffle(&mut rng);

    // Media stories are otherwise specific to the broadcast where they create narratives
    // There is a main broacast in english that appears on many co-streamer channels, 
    // thus there is a 25% chance that a new story comes from the broadcast
    if rng.random_range(0..4) == 0 {
        messages.push(Some(lec_broadcast_media_story(
                msg_id,
                is_positive,
                centered_on_player,
                team,
                player,
                date,
            )));
    }
    return messages.into_iter().flatten().next()?;
}

fn spanish_media_story(
    msg_id: &str,
    is_positive: bool,
    centered_on_player: bool,
    team: &Team,
    player: &Player,
    today: &str,
) -> Option<InboxMessage> {
    let mut rng = rand::rng();
    let mut messages = Vec::new(); 

    // Media stories can be podcasts broacasted on specific days
    if let Ok(date) = NaiveDate::parse_from_str(&today, "%Y-%m-%d") {
        if date.weekday() == Weekday::Tue || date.weekday() == Weekday::Wed || date.weekday() == Weekday::Fri  {
            // Al Lio Podcast
            messages.push(Some(al_lio_podcast_media_story(
                msg_id,
                is_positive,
                centered_on_player,
                team,
                player,
                today,
            )));
        }
        // Esportmaniacos go on live very almost everyday so we add a random limit on every day to not spam the player
        if rng.random_bool(0.25) {
            // Esportmaniacos
            messages.push(Some(esportmaniacos_podcast_media_story(
                    msg_id,
                    is_positive,
                    centered_on_player,
                    team,
                    player,
                    today,
                )));
        }
    }
    // Shuffle the messages and keep only one (ingored if empty)
    messages.shuffle(&mut rng);

    // Media stories are otherwise specific to the broadcast where they create narratives
    // There is a main broacast in spanish that appears on co-streamer channel but with a limited audience, 
    // thus there is a 10% chance that a new story comes from the broadcast
    if rng.random_range(0..10) == 0 {
        messages.push(Some(les_broadcast_media_story(
                msg_id,
                is_positive,
                centered_on_player,
                team,
                player,
                today,
            )));
    }
    return messages.into_iter().flatten().next()?;
}

fn french_media_story(
    msg_id: &str,
    is_positive: bool,
    centered_on_player: bool,
    team: &Team,
    player: &Player,
    today: &str,
) -> Option<InboxMessage> {
    let mut rng = rand::rng();
    let mut messages = Vec::new(); 

    // Media stories can be podcasts broadcasted on specific days
    if let Ok(date) = NaiveDate::parse_from_str(&today, "%Y-%m-%d") {
        if date.weekday() == Weekday::Wed {
            let mut rng = rand::rng();
            let weights = [70, 15, 15]; // Dans le Carré has more hype than the other french podcasts
            let dist = WeightedIndex::new(&weights).unwrap();
            let selected_index = dist.sample(&mut rng);

            match selected_index {
                0 => messages.push(Some(dans_le_carre_podcast_media_story(
                        msg_id,
                        is_positive,
                        centered_on_player,
                        team,
                        player,
                        today,
                    ))),
                1 => messages.push(Some(en_bref_podcast_media_story(
                        msg_id,
                        is_positive,
                        centered_on_player,
                        team,
                        player,
                        today,
                    ))),
                2 => messages.push(Some(stopwatch_podcast_media_story(
                        msg_id,
                        is_positive,
                        centered_on_player,
                        team,
                        player,
                        today,
                    ))),
                _ => unreachable!(),
            };            
        }
    }
    // Shuffle the messages and keep only one (ingored if empty)
    messages.shuffle(&mut rng);

    // Media stories are otherwise specific to the broadcast where they create narratives
    // There is a main broacast in spanish that appears on many co-streamer channel and is huge, 
    // thus there is a 50% chance that a new story comes from the broadcast
    if rng.random_range(0..2) == 0 {
        messages.push(Some(otplol_broadcast_media_story(
                msg_id,
                is_positive,
                centered_on_player,
                team,
                player,
                today,
            )));
    }
    return messages.into_iter().flatten().next()?;
}

fn german_streamer_message(
    msg_id: &str,
    is_positive: bool,
    team: &Team,
    player: &Player,
    today: &str,
) -> InboxMessage {
    let mut rng = rand::rng();
    let weights = [80, 10, 10, 10]; // Based on twitch tracker and estimation
    let mut dist = WeightedIndex::new(&weights).unwrap();
    let mut selected_index = dist.sample(&mut rng);

    let mut msg :Option<InboxMessage> = None;
    while msg.is_none() {
        match selected_index {
            0 => msg = tolkin_stream_message(&msg_id, is_positive, team, player, &today),
            1 => msg = obsess_stream_message(&msg_id, is_positive, team, player, &today),
            2 => msg = karni_stream_message(&msg_id, is_positive, team, player, &today),
            3 => msg = sola_stream_message(&msg_id, is_positive, team, player, &today),
            _ => unreachable!(),
        };
        if msg.is_none() {
            dist.update_weights(&[(selected_index, &0)]).unwrap();
            if dist.total_weight() == 0 { unreachable!() }
            selected_index = dist.sample(&mut rng);
        }
    }
    msg.unwrap()
}

fn english_streamer_message(
    msg_id: &str,
    is_positive: bool,
    team: &Team,
    player: &Player,
    today: &str,
) -> InboxMessage {
    let mut rng = rand::rng();
    let weights = [75, 15, 5, 5]; // Based on twitch tracker and estimation
    let mut dist = WeightedIndex::new(&weights).unwrap();
    let mut selected_index = dist.sample(&mut rng);

    let mut msg :Option<InboxMessage> = None;
    while msg.is_none() {
        match selected_index {
            0 => msg = caedrel_stream_message(&msg_id, is_positive, team, player, &today),
            1 => msg = jankos_stream_message(&msg_id, is_positive, team, player, &today),
            2 => msg = gtroubleinc_stream_message(&msg_id, is_positive, team, player, &today),
            3 => msg = caltys_stream_message(&msg_id, is_positive, team, player, &today),
            _ => unreachable!(),
        };
        if msg.is_none() {
            dist.update_weights(&[(selected_index, &0)]).unwrap();
            if dist.total_weight() == 0 { unreachable!() }
            selected_index = dist.sample(&mut rng);
        }
    }
    msg.unwrap()
}

fn spanish_streamer_message(
    msg_id: &str,
    is_positive: bool,
    team: &Team,
    player: &Player,
    today: &str,
) -> InboxMessage {
    let mut rng = rand::rng();
    let weights = [70, 10, 10, 5, 5]; // Based on twitch tracker and estimation
    let mut dist = WeightedIndex::new(&weights).unwrap();
    let mut selected_index = dist.sample(&mut rng);

    let mut msg :Option<InboxMessage> = None;
    while msg.is_none() {
        match selected_index {
            0 => msg = ibai_stream_message(&msg_id, is_positive, team, player, &today),
            1 => msg = skain_streamer_message(&msg_id, is_positive, team, player, &today),
            2 => msg = werlyb_stream_message(&msg_id, is_positive, team, player, &today),
            3 => msg = getflakked_stream_message(&msg_id, is_positive, team, player, &today),
            4 => msg = el_yuste_stream_message(&msg_id, is_positive, team, player, &today),
            _ => unreachable!(),
        };
        if msg.is_none() {
            dist.update_weights(&[(selected_index, &0)]).unwrap();
            if dist.total_weight() == 0 { unreachable!() }
            selected_index = dist.sample(&mut rng);
        }
    }
    msg.unwrap()
}

fn french_streamer_message(
    msg_id: &str,
    is_positive: bool,
    team: &Team,
    player: &Player,
    today: &str,
) -> InboxMessage {
    let mut rng = rand::rng();
    let weights = [60, 15, 10, 10, 5]; // Based on twitch tracker and estimation
    let mut dist = WeightedIndex::new(&weights).unwrap();
    let mut selected_index = dist.sample(&mut rng);

    let mut msg :Option<InboxMessage> = None;
    while msg.is_none() {
        match selected_index {
            0 => msg = kameto_stream_message(&msg_id, is_positive, team, player, &today),
            1 => msg = trayton_stream_message(&msg_id, is_positive, team, player, &today),
            2 => msg = zaboutine_stream_message(&msg_id, is_positive, team, player, &today),
            3 => msg = skyyart_stream_message(&msg_id, is_positive, team, player, &today),
            4 => msg = peaxy_stream_message(&msg_id, is_positive, team, player, &today),
            _ => unreachable!(),
        };
        if msg.is_none() {
            dist.update_weights(&[(selected_index, &0)]).unwrap();
            if dist.total_weight() == 0 { unreachable!() }
            selected_index = dist.sample(&mut rng);
        }
    }
    msg.unwrap()
}

fn portuguese_streamer_message(
    msg_id: &str,
    is_positive: bool,
    team: &Team,
    player: &Player,
    today: &str,
) -> InboxMessage {
    let mut rng = rand::rng();
    let weights = [50, 50]; // Based on twitch tracker and estimation
    let mut dist = WeightedIndex::new(&weights).unwrap();
    let mut selected_index = dist.sample(&mut rng);

    let mut msg :Option<InboxMessage> = None;
    while msg.is_none() {
        match selected_index {
            0 => msg = archarom_stream_message(&msg_id, is_positive, team, player, &today),
            1 => msg = kamus_stream_message(&msg_id, is_positive, team, player, &today),
            _ => unreachable!(),
        };
        if msg.is_none() {
            dist.update_weights(&[(selected_index, &0)]).unwrap();
            if dist.total_weight() == 0 { unreachable!() }
            selected_index = dist.sample(&mut rng);
        }
    }
    msg.unwrap()
}

fn brazilian_streamer_message(
    msg_id: &str,
    is_positive: bool,
    team: &Team,
    player: &Player,
    today: &str,
) -> InboxMessage {
    // No brazilian streamer for the LEC which is the only league supported for the moment
    // Relay english streamers for now as portuguese and brazilian communities are
    // completely separate according to brazilians.
    // Add league-dependant conditions when CBLoL is added
    english_streamer_message(msg_id, is_positive, team, player, today)
}

fn turkish_streamer_message(
    msg_id: &str,
    is_positive: bool,
    team: &Team,
    player: &Player,
    today: &str,
) -> InboxMessage {
    let mut rng = rand::rng();
    let weights = [90, 10]; // Based on twitch tracker and estimation
    let mut dist = WeightedIndex::new(&weights).unwrap();
    let mut selected_index = dist.sample(&mut rng);

    let mut msg :Option<InboxMessage> = None;
    while msg.is_none() {
        match selected_index {
            0 => msg = lynxcerezlol_stream_message(&msg_id, is_positive, team, player, &today),
            1 => msg = halpern_stream_message(&msg_id, is_positive, team, player, &today),
            _ => unreachable!(),
        };
        if msg.is_none() {
            dist.update_weights(&[(selected_index, &0)]).unwrap();
            if dist.total_weight() == 0 { unreachable!() }
            selected_index = dist.sample(&mut rng);
        }
    }
    msg.unwrap()
}

fn italian_streamer_message(
    msg_id: &str,
    is_positive: bool,
    team: &Team,
    player: &Player,
    today: &str,
) -> InboxMessage {
    let mut rng = rand::rng();
    let weights = [90, 10]; // Based on twitch tracker and estimation
    let mut dist = WeightedIndex::new(&weights).unwrap();
    let mut selected_index = dist.sample(&mut rng);

    let mut msg :Option<InboxMessage> = None;
    while msg.is_none() {
        match selected_index {
            0 => msg = brizz94_stream_message(&msg_id, is_positive, team, player, &today),
            1 => msg = terenas_stream_message(&msg_id, is_positive, team, player, &today),
            _ => unreachable!(),
        };
        if msg.is_none() {
            dist.update_weights(&[(selected_index, &0)]).unwrap();
            if dist.total_weight() == 0 { unreachable!() }
            selected_index = dist.sample(&mut rng);
        }
    }
    msg.unwrap()
}

fn podcast_media_story_message(
    msg_id: &str,
    is_positive: bool,
    centered_on_player: bool,
    podcast_name: &str,
    host_names: Vec<&str>,
    guest_names: Vec<&str>,
    team: &Team,
    player: &Player,
    date: &str,
) -> InboxMessage {
    let mut rng = rand::rng();

    let host_name = host_names[rng.random_range(0..host_names.len())];
    let guest_name;
    if guest_names.len() > 0 {
        guest_name = guest_names[rng.random_range(0..guest_names.len())];
    } else {
        // Get a different host as a guest for the second slot is there are no 
        // rotating guests or main host in this podcast
        guest_name = loop {
            let name = host_names[rng.random_range(0..host_names.len())];
            if name != host_name {
                break name;
            }
        };
    }
    let player_name = &player.match_name;
    // let player_position = &player.position;
    let team_name = &team.name;
    let team_short_name = &team.short_name;

    let (subject, body, subject_key, body_key);

    if centered_on_player { // Stories centered on a specific player
        let variant = rng.random_range(0..6usize);
        (subject, body, subject_key, body_key) = if is_positive {
            match variant {
                0 => (
                    format!("{} — Focus on {}", podcast_name, player_name),
                    format!(
                        "{} asked {} about {}'s performances recently: \
                        \"I could not think about a better player in the league right now at this role. \
                        I mean, look at the CS diff and the gold diff at 15 minutes, it speaks for itself. \
                        A {} in a good shape gives a dominant {}\".\n\n\
                        Make sure to use {}'s full potential.",
                        host_name, guest_name, player_name, player_name, team_short_name, player_name
                    ),
                    "be.msg.podcast.player.positive.subject0",
                    "be.msg.podcast.player.positive.body0",
                ),
                1 => (
                    format!("{} — {} saves {}", podcast_name, player_name, team_short_name),
                    format!(
                        "{} has been particularly adamant about the impact of {}: \
                        \"Let's be clear. {} plays badly. They should be doing better than that. \
                        Yet their results are still okay and it is entirely because of {}. \
                        If it was not for the prime faker-level performances we saw, these \
                        games would never be winnable. {} saves {}. Period\".\n\n\
                        Pay attention to the champions {} wants to play during the draft phase.",
                        guest_name, player_name, team_name, player_name, player_name, team_name, player_name
                    ),
                    "be.msg.podcast.player.positive.subject1",
                    "be.msg.podcast.player.positive.body1",
                ),
                2 => (
                    format!("{} — {} is playing good", podcast_name, player_name),
                    format!(
                        "{} highlighted {} this week: \"I was surprised by how {} played on the map recently. {} as \
                        a whole is perfectly in sync with {} and each play made on one side of the map is perfectly \
                         respected at the other. They completely out-tempo their oponnents and that comes from the \
                         pressure {} imposes. The only thing that separates them from the best teams right now are \
                          teamfights and a bit of temperance in late-game to avoid throws\".\n\n\
                        Should we focus on refining our strengths or working on our weaknesses?",
                        guest_name, player_name, player_name, team_short_name, player_name, team_short_name
                    ),
                    "be.msg.podcast.player.positive.subject2",
                    "be.msg.podcast.player.positive.body2",
                ),
                3 => (
                    format!("{} — Improvement from {}", podcast_name, player_name),
                    format!(
                        "{} wanted to talk about {} recent performances in this week episode: \
                        \"You can tell when a player strives to get better results. However, I feel like \
                        a lot of observers forget to mention improvements when they come. So I want to highlight \
                        {} as I saw a drastic change in terms of in-game behavior. You can sense different clicks, \
                        sharp decisions and almost no missed skillshots\".\n\n\
                        {} could become our star player in the future.",
                        guest_name, player_name, player_name, player_name
                    ),
                    "be.msg.podcast.player.positive.subject3",
                    "be.msg.podcast.player.positive.body3",
                ),
                4 => (
                    format!("{} praises {} — the desk is unanimous", podcast_name, player_name),
                    format!(
                        "The {} panel spent a good chunk of today's stream on {} at {}: 
                        \"Nothing to criticize this week. The guy is absolutely dominating\".\n\n\
                        The positive coverage should give confidence in the locker room.",
                        podcast_name, player_name, team_short_name
                    ),
                    "be.msg.podcast.player.positive.subject4",
                    "be.msg.podcast.player.positive.body4",
                ),
                _ => (
                    format!("{} — {} is carrying {} this split", podcast_name, player_name, team_short_name
                    ),
                    format!(
                        "The {} panel rarely agree on anything, but {} got a full pass today: \
                        \"Consistent, solid, no dips in form. One of the best players in the league right now. \
                        {} should do everything to keep him\".\n\n\
                        Good for morale — and for the market value.",
                        podcast_name, player_name, team_short_name
                    ),
                    "be.msg.podcast.player.positive.subject5",
                    "be.msg.podcast.player.positive.body5",
                )
            }
        } else {
            match variant {
                0 => (
                    format!("{} — {} bad performances", podcast_name, player_name),
                    format!(
                        "{} wanted to talk about {} recent performances in this week episode: \
                        \"I don't know what's happenning behind the scenes but I have to say that I don't recognize \
                        {} anymore. The plays, the interviews. It feels like there is no confidence anymore. \
                        What is happening in {}?\".\n\n\
                        We should speak with {}.",
                        guest_name, player_name, player_name, team_short_name, player_name
                    ),
                    "be.msg.podcast.player.negative.subject0",
                    "be.msg.podcast.player.negative.body0",
                ),
                1 => (
                    format!("{} — The competition is too strong for {}", podcast_name, player_name),
                    format!(
                        "{} nuanced {} bad performances considering the level of the competition: \
                        \"Honestly, {} does not play badly. There are good ideas, good trades, good macro movement. \
                        It is simply not enough. The competition at this position is way too fierce and {} struggles \
                        to improve. {} should do better with another player but I still think that {} \
                        deserves to play at this level\".\n\n\
                        We should maybe appoint another positional coach dedicated to {}.",
                        guest_name, player_name, player_name, player_name, team_short_name, player_name, player_name
                    ),
                    "be.msg.podcast.player.negative.subject1",
                    "be.msg.podcast.player.negative.body1",
                ),
                2 => (
                    format!("{} — {} is the WORST player I have ever seen", podcast_name, player_name),
                    format!(
                        "{} has been particularly critical towards {} this week: \
                        \"{} is the WORST player I have ever seen. What was that performance seriously? Are you saying \
                        there was not any player available in the off-season that could hold a mouse? At this point, \
                        {}'s coaching staff can't do miracles. I swear, if {} does not change {} for another player at \
                        the end of the split, it will be a total slap in the face to the fans\".\n\n\
                        We should consider the various options for the position of {}.",
                        guest_name, player_name, player_name, team_short_name, team_short_name, player_name, player_name
                    ),
                    "be.msg.podcast.player.negative.subject2",
                    "be.msg.podcast.player.negative.body2",
                ),
                3 => (
                    format!("{} — The meta is bad for {}", podcast_name, player_name),
                    format!(
                        "{} mentionned the impact of the meta on {}'s form: \
                        \"I think that {} players are better than what they are showing right now, especially {}. \
                        I mean, {}'s champion pool is quite one-sided and corresponded perfectly to the meta \
                        but with all those spicy picks coming and the changes made to the items and the runes, {} \
                        is caught of guard and they have to find a whole new balance. As I see it, there are only two options. \
                        Either {} manages to adapt or {} has to forget about the meta and go back to what made them good\".\n\n\
                        We should think about it.",
                        guest_name, player_name, team_short_name, player_name, player_name, team_short_name, player_name, team_short_name
                    ),
                    "be.msg.podcast.player.subject3",
                    "be.msg.podcast.player.body3",
                ),
                4 => (
                    format!("{} goes after {} — Panel shows no mercy", podcast_name, player_name),
                    format!("The {} panel did not hold back today on {}: \
                        \"The guy has completely disappeared. What happened during this split? \
                        {} can't keep relying on someone performing like this\".\n\n\
                        This kind of coverage tends to hit morale hard. Worth having a word with {}.",
                        podcast_name, player_name, team_short_name, player_name
                    ),
                    "be.msg.podcast.player.negative.subject4",
                    "be.msg.podcast.player.negative.body4",
                ),
                _ => (
                    format!("{} questions {}'s consistency", podcast_name, player_name),
                    format!("Today's {} session turned into a full breakdown of {}'s recent form. \
                        The verdict: \"Inconsistent. No regularity. Some days at top level, others completely invisible. \
                        {} deserves better output from someone in that role\".\n\n\
                        Keep an eye on the player's morale.",
                        podcast_name, player_name, team_short_name
                    ),
                    "be.msg.podcast.player.negative.subject5",
                    "be.msg.podcast.player.negative.body5",
                ),
            }
        }
    } else { // Stories not centered on a specific player
        let variant = rng.random_range(0..4usize);
        (subject, body, subject_key, body_key) = if is_positive {
            match variant {
                0 => (
                    format!("{} — {}'s coaching staff", podcast_name, team_short_name),
                    format!(
                        "In this week episode {} spent time talking about {}'s coaching staff: \
                        \"To be honest, I think I never saw such a drafting style before. It seems like their \
                        coaching staff keeps inventing new champions, new roles to play them in. \
                        It is both effective and particularly enjoyable to watch !\".\n\n\
                        We really want to keep our coaching staff. They are doing a great job.",
                        host_name, team_short_name
                    ),
                    "be.msg.podcast.general.positive.subject0",
                    "be.msg.podcast.general.positive.body0",
                ),
                1 => (
                    format!("{} — {} is HYPE", podcast_name, team_name),
                    format!(
                        "{} shared how much {} builds hype in the league: 
                        \"I think I have not been this hyped for a long time. I know that people might think \
                        I am crazy but I am serious. The way {} plays and drafts makes me think they can \
                        challenge the top of the world. And I am not alone. Look at the excitement \
                        on social medias. The fans are getting louder, the hype is growing. I am sure they are \
                        selling more jerseys than they ever did. {} should really do everything they can to foster \
                        this momentum and become one of the best\".\n\n\
                        This is a very good sign for us. I will make sure to check the impact on our sales.",
                        guest_name, team_short_name, team_short_name, team_short_name
                    ),
                    "be.msg.podcast.general.positive.subject1",
                    "be.msg.podcast.general.positive.body1",
                ),
                2 => (
                    format!("{} — {} players seem so cool", podcast_name, team_short_name),
                    format!(
                        "{} talked about {}'s content: \"I think that {}'s media team is underrated. \
                        Their content is fresh, they have good ideas and they keep finding new ways to make \
                        the players loveable. The league should draw their aspiration from them\".\n\n\
                        I just wanted to let you know. Sometimes, the players don't want to, but it \
                        has a positive impact on {}.",
                        guest_name, team_short_name, team_short_name, team_name
                    ),
                    "be.msg.podcast.general.positive.subject2",
                    "be.msg.podcast.general.positive.body2",
                ),
                _ => (
                    format!("{} — {} has so many good players", podcast_name, team_short_name),
                    format!(
                        "{} particularly emphasized the importance of synergy \
                        when building a roster like {}'s: \
                        \"You saw many superteams in the past crumbling under the pressure and the egos. \
                        {} is not one of them. They have very talented players, among the best in the league \
                          and currently they are all in a top shape. But the difference comes from the team plays. \
                        Watch the jungler pathing, look at the wards, pay attention to the pings\".\n\n\
                        Other teams might be interested in our players. We should pay attention to their contracts.",
                        host_name, team_short_name, team_short_name
                    ),
                    "be.msg.podcast.general.positive.subject3",
                    "be.msg.podcast.general.positive.body3",
                ),
            }
        } else {
            match variant {
                0 => (
                    format!("{} — Hot Take: {} will crumble", podcast_name, team_short_name),
                    format!(
                        "{} shared a hot take about {}: \
                        \"We all saw the same games this weekend. Uncertain lanes, pointless 50/50s on objectives, \
                        weird deaths on the sidelanes. For me it is obvious that {} will crumble in the near future. \
                        Even {} had a bad performance in their last match\".\n\n\
                        We should maybe change our training methods?",
                        guest_name, team_short_name, team_short_name, player_name
                    ),
                    "be.msg.podcast.general.negative.subject0",
                    "be.msg.podcast.general.negative.body0",
                ),
                1 => (
                    format!("{} — {} is a JOKE ", podcast_name, team_short_name),
                    format!(
                        "{} pointed fingers at us: \
                        \"What is {}'s management doing ?! They have a coaching staff drating nonsense, players \
                        yawning on stage and an off-season beyond understanding ! Is this supposed to represented \
                        the best of the best? Sell the slot already !\".\n\n\
                        In terms of budget, what can we do to repair the situation?",
                        guest_name, team_name
                    ),
                    "be.msg.podcast.general.negative.subject1",
                    "be.msg.podcast.general.negative.body1",
                ),
                2 => (
                    format!("{} — What can {} do to make a comeback?", podcast_name, team_short_name),
                    format!(
                        "{} asked {} what could {} do to make a comeback: \
                        \"It's difficult you know. I can only have compassion with {}'s coaching staff and players. \
                        I am sure they are doing their best but maybe they should prioritize scrimming \
                        against teams from other leagues, even minor ones. This could help them build a momentum \
                        and play with trust. In this situation, your biggest ennemy is yourself\".\n\n\
                        We could maybe adapt our training schedule?",
                        host_name, guest_name, team_short_name, team_short_name
                    ),
                    "be.msg.podcast.general.negative.subject2",
                    "be.msg.podcast.general.negative.body2",
                ),
                _ => (
                    format!("{} — {}'s coaching staff", podcast_name, team_short_name),
                    format!(
                        "In this week episode {} spent time talking about {}'s coaching staff: \
                        \"I don't understand how these people can still have a job right now. The way they drafted \
                        last weekend? Am I watching tier 3 League of Legends? It is painfull to watch. \
                        Give your players a real staff if you really want to win !\".\n\n\
                        We should pay attention to this kind of comment. These guests are knowledgeable about the game and they have influence over their audience.",
                        host_name, team_short_name
                    ),
                    "be.msg.podcast.general.negative.subject3",
                    "be.msg.podcast.general.negative.body3",
                ),
            }
        }
    } 

    let msg: InboxMessage = InboxMessage::new(
        msg_id.to_string(),
        subject,
        body,
        "Media team".to_string(),
        date.to_string(),
    )
    .with_category(MessageCategory::Media)
    .with_priority(MessagePriority::Normal)
    .with_sender_role("Lead Communication")
    .with_action(action(
        "ack",
        "Noted",
        "be.msg.event.ack",
        ActionType::Acknowledge,
    ))
    .with_i18n(
        subject_key,
        body_key,
        params(&[
            ("player", player_name),
            ("team", team_name),
            ("team_short", team_short_name),
            ("podcast", podcast_name),
            ("host", host_name),
            ("guest", guest_name)]),
    )
    .with_sender_i18n("be.sender.media", "be.role.leadCom")
    .with_context(MessageContext {
        player_id: Some(player.id.clone()),
        ..Default::default()
    });
    msg
}

pub(super) fn hopeum_podcast_media_story(
    msg_id: &str,
    is_positive: bool,
    centered_on_player: bool,
    team: &Team,
    player: &Player,
    date: &str,
) -> Option<InboxMessage> {
    let podcast_name = "HopEUm";
    let host_names = vec!["Quickshot", "Kitty"];
    let guest_names = vec![
        "Jankos", "Trayton", "Brizz94", "Caltys", 
        "SendOo", "Skyyart", "Sola", "Wadid",
    ];

    // Add special messages here if relevant to create more engaging stories
    // Use randomness to select them or a default_stream_message
    return Some(podcast_media_story_message(
        msg_id, 
        is_positive, 
        centered_on_player,
        podcast_name,
        host_names,
        guest_names,
        team,
        player,
        date));
}

pub(super) fn mind_the_gap_podcast_media_story(
    msg_id: &str,
    is_positive: bool,
    centered_on_player: bool,
    team: &Team,
    player: &Player,
    date: &str,
) -> Option<InboxMessage> {
    let podcast_name = "Mind the Gap";
    let host_names = vec!["Vedius", "Jatt"];
    let guest_names = vec![];

    // Add special messages here if relevant to create more engaging stories
    // Use randomness to select them or a default_stream_message
    return Some(podcast_media_story_message(
        msg_id, 
        is_positive, 
        centered_on_player,
        podcast_name,
        host_names,
        guest_names,
        team,
        player,
        date));
}

pub(super) fn lec_podcast_media_story(
    msg_id: &str,
    is_positive: bool,
    centered_on_player: bool,
    team: &Team,
    player: &Player,
    date: &str,
) -> Option<InboxMessage> {
    let podcast_name = "The LEC Podcast";
    let host_names = vec!["Odoamne","Finn","Jackspectra"];
    let guest_names = vec![];

    // Add special messages here if relevant to create more engaging stories
    // Use randomness to select them or a default_stream_message
    return Some(podcast_media_story_message(
        msg_id, 
        is_positive, 
        centered_on_player,
        podcast_name,
        host_names,
        guest_names,
        team,
        player,
        date));
}

pub(super) fn esportmaniacos_podcast_media_story(
    msg_id: &str,
    is_positive: bool,
    centered_on_player: bool,
    team: &Team,
    player: &Player,
    date: &str,
) -> Option<InboxMessage> {
    let podcast_name = "Esportmaníacos";
    let host_names = vec!["Eros"];
    let guest_names = vec!["Axineas", "Gila", "Irene"];

    // Add special messages here if relevant to create more engaging stories
    // Use randomness to select them or a default_stream_message
    return Some(podcast_media_story_message(
        msg_id, 
        is_positive, 
        centered_on_player,
        podcast_name,
        host_names,
        guest_names,
        team,
        player,
        date));
}

pub(super) fn al_lio_podcast_media_story(
    msg_id: &str,
    is_positive: bool,
    centered_on_player: bool,
    team: &Team,
    player: &Player,
    date: &str,
) -> Option<InboxMessage> {
    let podcast_name = "Al Lío Podcast";
    let host_names = vec!["Eros"];
    let guest_names = vec!["Axineas", "Gila", "Irene"];

    // Add special messages here if relevant to create more engaging stories
    // Use randomness to select them or a default_stream_message
    return Some(podcast_media_story_message(
        msg_id, 
        is_positive, 
        centered_on_player,
        podcast_name,
        host_names,
        guest_names,
        team,
        player,
        date));
}

pub(super) fn dans_le_carre_podcast_media_story(
    msg_id: &str,
    is_positive: bool,
    centered_on_player: bool,
    team: &Team,
    player: &Player,
    date: &str,
) -> Option<InboxMessage> {
    let podcast_name = "Dans le Carré";
    let host_names = vec!["Drako"];
    let guest_names = vec!["Hugo", "Ethan"];

    // Add special messages here if relevant to create more engaging stories
    // Use randomness to select them or a default_stream_message
    return Some(podcast_media_story_message(
        msg_id, 
        is_positive, 
        centered_on_player,
        podcast_name,
        host_names,
        guest_names,
        team,
        player,
        date));
}

pub(super) fn en_bref_podcast_media_story(
    msg_id: &str,
    is_positive: bool,
    centered_on_player: bool,
    team: &Team,
    player: &Player,
    date: &str,
) -> Option<InboxMessage> {
    let podcast_name = "En Bref";
    let host_names = vec!["Lucas"];
    let guest_names = vec!["Ilyas", "Mirai", "5D"];

    // Add special messages here if relevant to create more engaging stories
    // Use randomness to select them or a default_stream_message
    return Some(podcast_media_story_message(
        msg_id, 
        is_positive, 
        centered_on_player,
        podcast_name,
        host_names,
        guest_names,
        team,
        player,
        date));
}

pub(super) fn stopwatch_podcast_media_story(
    msg_id: &str,
    is_positive: bool,
    centered_on_player: bool,
    team: &Team,
    player: &Player,
    date: &str,
) -> Option<InboxMessage> {
    let podcast_name = "Stopwatch";
    let host_names = vec!["Lilian", "Lazer", "Calo"];
    let guest_names = vec![];

    // Add special messages here if relevant to create more engaging stories
    // Use randomness to select them or a default_stream_message
    return Some(podcast_media_story_message(
        msg_id, 
        is_positive, 
        centered_on_player,
        podcast_name,
        host_names,
        guest_names,
        team,
        player,
        date));
}

fn broadcast_media_story_message(
    msg_id: &str,
    is_positive: bool,
    centered_on_player: bool,
    broadcast_name: &str,
    host_names: Vec<&str>,
    guest_names: Vec<&str>,
    team: &Team,
    player: &Player,
    date: &str,
) -> InboxMessage {
    let mut rng = rand::rng();
    let variant = rng.random_range(0..4usize);

    let host_name = host_names[rng.random_range(0..host_names.len())];
    let guest_name;
    if guest_names.len() > 0 {
        guest_name = guest_names[rng.random_range(0..guest_names.len())];
    } else {
        // Get a different host as a guest for the second slot is there are no 
        // rotating guests or main host in this broadcast
        guest_name = loop {
            let name = host_names[rng.random_range(0..host_names.len())];
            if name != host_name {
                break name;
            }
        };
    }
    let player_name = &player.match_name;
    // let player_position = &player.position;
    let team_name = &team.name;
    let team_short_name = &team.short_name;

    let (subject, body, subject_key, body_key);

    if centered_on_player { // Stories centered on a specific player
        (subject, body, subject_key, body_key) = if is_positive {
            match variant {
                0 => (
                    format!("{} — {} dominates the statistics", broadcast_name, player_name),
                    format!(
                        "{} spent time comparing {} with the other competitors: \
                        \"We checked the data and I was flabbergasted by {}'s statistics. \
                        In terms of jungle proximity and presence on early objectives there \
                        is not any player that performs better. And the impact in the early \
                        game is clear. {} has a better setup on early objectives and it gives \
                        either free kills or free objectives.\n\n\
                        We need to make sure {}'s morale stays up.",
                        guest_name, player_name, player_name, team_short_name, player_name
                    ),
                    "be.msg.broadcast.player.positive.subject0",
                    "be.msg.broadcast.player.positive.body0",
                ),
                1 => (
                    format!("{} — {}'s play analyzed on broadcast", broadcast_name, player_name),
                    format!(
                        "{} analyzed {}'s play on the {} broadcast: \
                        \"As you can see right here, {} finds the angle to put a pink ward in this bush \
                        and just a few seconds after, it hides the flank. This ward is a game changer and at\
                        the moment {} decides to turn, it's yet again {} that goes in and clutch the teamfight\".\n\n\
                        This kind of work is rarely emphasized. Make sure to mention it to the coaching staff.",
                        guest_name, player_name, broadcast_name, player_name, team_name, player_name
                    ),
                    "be.msg.broadcast.player.positive.subject1",
                    "be.msg.broadcast.player.positive.body1",
                ),
                2 => (
                    format!("{} — {} live interview", broadcast_name, player_name),
                    format!(
                        "{} interviewed {} this week: \
                        \n{} - \"You played particularly good today, was there something personal \
                         against today's opponent?\"\
                        \n{} - \"Well, kind of. In scrims, we have been struggling to play against \
                         them and it was mainly my fault. I had an ego check and I really wanted \
                         to win so I poured a lot of me into preparing today's matchups\".\n\n\
                        {} is doing a lot of efforts to improve.",
                        host_name, player_name, host_name, player_name, player_name
                    ),
                    "be.msg.broadcast.player.positive.subject2",
                    "be.msg.broadcast.player.positive.body2",
                ),
                _ => (
                    format!("{} — Enthusiasm around {}", broadcast_name, player_name),
                    format!(
                        "The whole {} broadcast was enthustiast about {} recent performances \
                        this week. {} in particular, highlighted a lot of improvements: \
                        \"I did not expect {} to improve that much in such a short time. \
                        I think I never saw a player changing that fast. How far will {} be \
                        able to go with {}? How far {} will be able to grow with {}? \
                        I can wait to see what's coming for them\".\n\n\
                        The narrative is shifting!",
                        broadcast_name, player_name, guest_name, player_name, team_short_name, player_name, player_name, team_short_name
                    ),
                    "be.msg.broadcast.player.positive.subject3",
                    "be.msg.broadcast.player.positive.body3",
                ),
            }
        } else {
            match variant {
                0 => (
                    format!("{} — {} showed as bottom in statistics", broadcast_name, player_name),
                    format!(
                        "{} was walking on eggshells discussing {} statistics: \
                        \"{} is a good team but they can't win and statistically speaking, the data points \
                        towards a scapegoat. {} is last of the league in a lot of indicators and we saw \
                        huge mistakes in game. Usually, {} plays good but each time they are close to win, \
                        {} goes one step too far and the momentum shifts completely\".\n\n\
                        Do we have options to replace {}?",
                        guest_name, player_name, team_short_name, player_name, team_short_name, player_name, player_name
                    ),
                    "be.msg.broadcast.player.negative.subject0",
                    "be.msg.broadcast.player.negative.body0",
                ),
                1 => (
                    format!("{} — Detrimental narrative around {}", broadcast_name, team_name),
                    format!(
                        "{} criticized {} despite good results: \
                        \"Everybody completely overestimate {}. Have anyone paid attention to their vision setup?
                        The players' hands simply hide their awfull weaknesses. In a few weeks you will see I am right.\
                        {} will not able to take a single objective playing like that\".\n\n\
                        Is {} right on this?",
                        guest_name, team_short_name, team_short_name, team_short_name, guest_name
                    ),
                    "be.msg.broadcast.player.negative.subject1",
                    "be.msg.broadcast.player.negative.body1",
                ),
                2 => (
                    format!("{} — {} live interview", broadcast_name, player_name),
                    format!(
                        "{} has been hesitant in an interview with {}: \
                        \n{} - \"You team seems to keep playing with a weakside top. \
                        Is this a team style you decided as a team based on scrims?\"\
                        \n{} - \"I don't know. Not really. Maybe it's because I can't adapt fast enough on stage \
                        to what happens on the map. I don't think it is a conscious decision, at least not from me\".\n\n\
                        This is really a bad interview. We need to provide media training to {} as soon as possible.",
                        player_name, host_name, host_name, player_name, player_name
                    ),
                    "be.msg.broadcast.player.negative.subject2",
                    "be.msg.broadcast.player.negative.body2",
                ),
                _ => (
                    format!("{} — {} insulted on broadcast", broadcast_name, player_name),
                    format!(
                        "{} has been particularly harsh towards {} while casting {} games. During a teamfight \
                        where {} missed a skillshot, {} instantly shouted \"Bro how do you miss that?! You have one job.\" \
                        and completed the complaint with a \"pun\" based on {}'s pseudo I won't write here. This is \
                        not the first time {} targets {}. This is borderline harassment at this point. I contacted Riot to\
                        make sure this is the last time something like that happens on the {} broadcast.",
                        guest_name, player_name, team_short_name, player_name, guest_name, player_name, guest_name, player_name, broadcast_name
                    ),
                    "be.msg.broadcast.player.negative.subject3",
                    "be.msg.broadcast.player.negative.body3",
                ),
            }
        }
    } else { // Stories not centered on a specific player
        (subject, body, subject_key, body_key) = if is_positive {
            match variant {
                0 => (
                    format!("{} — {} fans praised on broadcast", broadcast_name, team_name),
                    format!(
                        "{} wanted to send a message to the league communities this week: \
                        \"To every fan that listens to me. Look at what {} fans are doing online and during events.\
                        Take inspiration from it, make it your own, refine it and shout louder for your team. We \
                        absolutely need this kind of hype that {} fans are brining. Nobody expected them to be that \
                        game changer and yet here they are. Wake up fans! Show your pride and make sure that next time,\
                        I praise you instead of them\".\n\n\
                        The fans are on fire. Give them the show they demand!",
                        guest_name, team_short_name, team_short_name
                    ),
                    "be.msg.broadcast.general.positive.subject0",
                    "be.msg.broadcast.general.positive.body0",
                ),
                1 => (
                    format!("{} — {} dominant in statistics", broadcast_name, team_short_name),
                    format!(
                        "{} pointed out {} dominance in statistics: \
                        \"I was curious about it so I checked. {} almost always has the first and second drake. They \
                        snowball hard on the botside of the map and their toplaner respects really well the weakside. \
                        When the midgame comes, they can either trade drakes for gold at the opposite or fight for an \
                        early soul depending on their draft. They have the agency to choose what they want to do \
                        on the map and that changes everything\".\n\n\
                        Pay attention to the team's training schedule. We need to be in peak condition at the right time.",
                        guest_name, team_name, team_short_name
                    ),
                    "be.msg.broadcast.general.positive.subject1",
                    "be.msg.broadcast.general.positive.body1",
                ),
                2 => (
                    format!("{} — Who can stop {}?", broadcast_name, team_name),
                    format!(
                        "On the {} broadcast, {} hosted a special segment around {}, emphasizing in particular the macro game \
                        and the impact of the jungler/support duo on the whole map. According to {}: \"{} is \
                        unstoppable right now. Their laners might be challenged but their jungle/support duo is
                        god-like in this meta\".\n\n\
                        I have put together some special content for fans who write fanfiction about our jungle-support duo.",
                        broadcast_name, guest_name, team_short_name, guest_name, team_name
                    ),
                    "be.msg.broadcast.general.positive.subject2",
                    "be.msg.broadcast.general.positive.body2",
                ),
                _ => (
                    format!("{} — {} preparation emphasized on broadcast", broadcast_name, team_short_name),
                    format!(
                        "The preparation of {} has been praised on the {} broadcast. {} mentioned a recent interview \
                        of {}'s headcoach where they said they had worked a lot on their preparation tools: \
                        \"Our analyst has done a huge job building our internal tools. I think that these tools are \
                        essential nowadays and of course they don't remplace long-term work and specific counter-picks \
                        but they help having a solid dauy-to-day baseline when building our drafts and preparing the scrims\".\n\n\
                        We need to check the contract of our analyst.",
                        team_short_name, broadcast_name, host_name, team_short_name
                    ),
                    "be.msg.broadcast.general.positive.subject3",
                    "be.msg.broadcast.general.positive.body3",
                ),
            }
        } else {
            match variant {
                0 => (
                    format!("{} — {} fans criticized on broadcast", broadcast_name, team_name),
                    format!(
                        "{} fans sparked {}'s anger during the {} broadcast: \
                        \"It's always {} fans that don't know the limits. Guys, I know you love your team but you
                        can't keep insulting the other communities. I will say out loud what others don't. The majority \
                        of your community is toxic and you should take responsability for the harassment you provoke\".\n\n\
                        This comment reignited a war on social media. Racist comments targetted {}. We can't let that happen.",
                        team_short_name, guest_name, broadcast_name, team_short_name, team_name
                    ),
                    "be.msg.broadcast.general.negative.subject0",
                    "be.msg.broadcast.general.negative.body0",
                ),
                1 => (
                    format!("{} — {} behind in statistics", broadcast_name, team_name),
                    format!(
                        "{} pointed fingers at {}: \
                        \"The truth hits hard but the statistics don't lie. No matter how much gold they get in \
                        the early game, {} can't win games. They give the impression of fighting back but they never secure \
                        objectives in mid-game and above all, they play in the dark. Look at the vision scores. How can \
                        they expect to win fights if they don't know where their opponents are on the map?\".\n\n\
                        Unfortunately, {} is right on that.",
                        guest_name, team_name, team_short_name, guest_name
                    ),
                    "be.msg.broadcast.general.negative.subject1",
                    "be.msg.broadcast.general.negative.body1",
                ),
                2 => (
                    format!("{} — Who can save {}?", broadcast_name, team_name),
                    format!(
                        "{} gave ideas in the {} broadcast about how {} could make a comeback: \
                        \"At this point, I think that {} should improve their mental coaching staff. \
                        There is a clear mental problem in their recent games. If they can't fix that, \
                        they will have to change players at the off-season to create a new momentum\".\n\n\
                        We could maybe adapt our training schedule?",
                        host_name, broadcast_name, team_short_name, team_short_name
                    ),
                    "be.msg.broadcast.general.negative.subject2",
                    "be.msg.broadcast.general.negative.body2",
                ),
                _ => (
                    format!("{} — {} preparation criticized on broadcast", broadcast_name, team_short_name),
                    format!(
                        "{} critized {}'s coaching staff on the {} broadcast this week: \
                        \"I don't have the data they have from the scrims but honestly at this moment of the \
                        draft, what picking that champion is criminal. It seems like a good idea matchup wise but \
                        overall their draft becomes incoherent. They can't go in, they can't receive, they do \
                        everything and nothing at the same time \".\n\n\
                        We should pay attention to the coherence of our drafts.",
                        host_name, team_short_name, broadcast_name
                    ),
                    "be.msg.broadcast.general.negative.subject3",
                    "be.msg.broadcast.general.negative.body3",
                ),
            }
        }
    } 

    let msg = InboxMessage::new(
        msg_id.to_string(),
        subject,
        body,
        "Media team".to_string(),
        date.to_string(),
    )
    .with_category(MessageCategory::Media)
    .with_priority(MessagePriority::Normal)
    .with_sender_role("Lead Communication")
    .with_action(action(
        "ack",
        "Noted",
        "be.msg.event.ack",
        ActionType::Acknowledge,
    ))
    .with_i18n(
        subject_key,
        body_key,
        params(&[
            ("player", player_name),
            ("team", team_name),
            ("team_short", team_short_name),
            ("broadcast", broadcast_name),
            ("host", host_name),
            ("guest", guest_name)]),
    )
    .with_sender_i18n("be.sender.media", "be.role.leadCom")
    .with_context(MessageContext {
        player_id: Some(player.id.clone()),
        ..Default::default()
    });
    msg
}

pub(super) fn lec_broadcast_media_story(
    msg_id: &str,
    is_positive: bool,
    centered_on_player: bool,
    team: &Team,
    player: &Player,
    date: &str,
) -> Option<InboxMessage> {
    let broadcast_name = "LEC";
    let host_names = vec!["Laure", "Sjokz"];
    let guest_names = vec!["Dagda", "Drakos", "Finn", "Hysterics", "Jackspektra", "Medic", "Odoamne", "Vedius"];

    // Add special messages here if relevant to create more engaging stories
    // Use randomness to select them or a default_stream_message
    return Some(broadcast_media_story_message(
        msg_id, 
        is_positive, 
        centered_on_player,
        broadcast_name,
        host_names,
        guest_names,
        team,
        player,
        date));
}

pub(super) fn les_broadcast_media_story(
    msg_id: &str,
    is_positive: bool,
    centered_on_player: bool,
    team: &Team,
    player: &Player,
    date: &str,
) -> Option<InboxMessage> {
    let broadcast_name = "LES";
    let host_names = vec!["Noa", "Fernando Cardenete", "Bebe"];
    let guest_names = vec![];

    // Add special messages here if relevant to create more engaging stories
    // Use randomness to select them or a default_stream_message
    return Some(broadcast_media_story_message(
        msg_id, 
        is_positive, 
        centered_on_player,
        broadcast_name,
        host_names,
        guest_names,
        team,
        player,
        date));
}

pub(super) fn otplol_broadcast_media_story(
    msg_id: &str,
    is_positive: bool,
    centered_on_player: bool,
    team: &Team,
    player: &Player,
    date: &str,
) -> Option<InboxMessage> {
    let broadcast_name = "OTPLoL";
    let host_names = vec!["Chips", "Noi", "Tweekz"];
    let guest_names = vec!["Chreak", "Marex", "Peaxy", "Yellowstar", "Blackbigo", "Glopo", "Splinter"];

    // Add special messages here if relevant to create more engaging stories
    // Use randomness to select them or a default_stream_message
    return Some(broadcast_media_story_message(
        msg_id, 
        is_positive, 
        centered_on_player,
        broadcast_name,
        host_names,
        guest_names,
        team,
        player,
        date));
}

pub(super) fn rumour_message(
    msg_id: &str,
    manager_lang: &str,
    team: &Team,
    player: &Player,
    date: &str,
) -> InboxMessage {
    // Rumours quickly spread accross reddit and twitter and can be relayed by many people
    // but the big source relayed by everyone in europe remains SheepEsports. I added a weight based on language
    // and impact for language-dependant sources and evaluated whether or not they had relays in other languages too
    let weights;
    match manager_lang {
        "DE" => weights = [95, 5, 0],
        "EN" => weights = [95, 5, 0],
        "ES" => weights = [50, 50, 0],
        "FR" => weights = [90, 10, 0],
        "PT" => weights = [95, 5, 0],
        "BR" => weights = [95, 5, 0],
        "TR" => weights = [45, 5, 60],
        "IT" => weights = [95, 10, 0],
        _ => weights = [100, 0, 0],
    }
    let mut rng = rand::rng();
    let mut dist = WeightedIndex::new(&weights).unwrap();
    let mut selected_index = dist.sample(&mut rng);

    let mut msg :Option<InboxMessage> = None;
    while msg.is_none() {
        match selected_index {
            0 => msg = sheep_esports_rumour_message(
                        &msg_id,
                        team,
                        player, 
                        &date),
            1 => msg = al_lio_rumour_message(
                        &msg_id,
                        team,
                        player, 
                        &date),
            2 => msg = duyum_tcl_rumour_message(
                        &msg_id,
                        team,
                        player, 
                        &date),
            _ => unreachable!(),
        };
        if msg.is_none() {
            dist.update_weights(&[(selected_index, &0)]).unwrap();
            if dist.total_weight() == 0 { unreachable!() }
            selected_index = dist.sample(&mut rng);
        }
    }
    msg.unwrap()
}

pub(super) fn default_rumour_message(
    msg_id: &str,
    rumour_media_name: &str,
    host_names: Vec<&str>,
    team: &Team,
    player: &Player,
    date: &str,
) -> InboxMessage {
    let mut rng = rand::rng();
    let variant = rng.random_range(0..10usize);

    let host_name = host_names[rng.random_range(0..host_names.len())];
    let player_name = &player.match_name;
    // let player_position = &player.position;
    let team_name = &team.name;
    let team_short_name = &team.short_name;

    let (subject, body, subject_key, body_key) = match variant {
        0 => (
            format!("{} — Transfer rumour: {} being tracked", rumour_media_name, player_name),
            format!("{} dropped a transfer hint today: \
                \"I'm hearing things about {}. Multiple teams have been asking questions. \
                Nothing confirmed yet, but the mercato around {} is heating up\".\n\n\
                Keep this in mind when planning your squad for next split.",
                host_name, player_name, team_short_name
            ),
            "be.msg.rumour.subject0",
            "be.msg.rumour.body0",
        ),
        1 => (
            format!("{} — Transfer rumour: {} might be benched", rumour_media_name, player_name),
            format!("{} dropped a transfer hint today: \
                \"Sources told me that {} should be benched at the end of the split. {} is currently \
                looking for their options in tier 2 but they have limited options. \
                I'll tell you more when I can\".\n\n\
                We should evaluate if we can take advantage of that situation.",
                host_name, player_name, team_short_name
            ),
            "be.msg.rumour.subject1",
            "be.msg.rumour.body1",
        ),
        2 => (
            format!("{} — Transfer rumour: {} poached by other teams", rumour_media_name, player_name),
            format!("{} mentionned {}'s contract today: \
                \"{} is having a hard time extending {}'s contract. Many people have been approaching \
                {} behind the scenes and Riot is turning a blind eye on this situation.\
                It is highly probable that {} will switch teams as a free agent when the {} \
                contract expires\".\n\n\
                This reminds us hard times.",
                host_name, player_name, team_short_name, player_name, player_name, player_name, team_short_name
            ),
            "be.msg.rumour.subject2",
            "be.msg.rumour.body2",
        ),
        3 => (
            format!("{} — Transfer rumour: discussions around {}", rumour_media_name, player_name),
            format!("{} mentionned active discussions around {}'s player: \
                \"People are talking with {} about {}. We are still far from having a real offer \
                on the table but teams are interested\".\n\n\
                We should take part in the discussions if it is not already the case.",
                host_name, team_short_name, team_short_name, player_name
            ),
            "be.msg.rumour.subject3",
            "be.msg.rumour.body3",
        ),
        4 => (
            format!("{} — Transfer rumour: {} envisionned by other teams", rumour_media_name, player_name),
            format!("{} hinted activity around {}'s contract: \
                \"I know people that talked to me about {}. Apparently, {} has received interest \
                from at least two other teams about {}. It is unsure if {} would want to \
                keep their player or act a roster change for now. I will let you know\".\n\n\
                Keep this in mind when planning your squad for next split.",
                host_name, player_name, player_name, team_short_name, player_name, team_short_name
            ),
            "be.msg.rumour.subject4",
            "be.msg.rumour.body4",
        ),
        5 => (
            format!("{} — {} hints at incoming deal for {}", rumour_media_name, host_name, team_name),
            format!("{} has been dropping hints about a incoming move: \
                \"I've been told there are conversations happening around {}. Something is being cooked. \
                I can't give names yet but stay tuned — this mercato is far from over\".\n\n\
                Could be noise, could be real. Worth watching.",
                host_name, team_short_name
            ),
            "be.msg.rumour.subject5",
            "be.msg.rumour.body5",
        ),
        6 => (
            format!("{} — League format leak, {} breaks it down", rumour_media_name, host_name),
            format!("Yesterday, {} revealed a potential leak: \
            \"I've been told the league is planning changes to the split format. Not confirmed, \
            but my sources are usually reliable. This could shake up how teams build their rosters\".\n\n\
            If true, this could affect your long-term planning.", host_name),
            "be.msg.rumour.subject6",
            "be.msg.rumour.body6",
        ),
        7 => (
            format!("{} — Big move incoming, league shaken up", rumour_media_name),
            format!("{} teased for incoming move: \
            \"There's a significant player movement about to happen in the league that nobody is talking about yet. \
            I'll just say this — some teams are going to have to rethink their rosters completely\".\n\n\
            Stay alert — the market is moving.", host_name),
            "be.msg.rumour.subject7",
            "be.msg.rumour.body7",
        ),
        8 => (
            format!("{} — {} likely to be swaped with another team", rumour_media_name, player_name),
            format!("{} revealed a potential roster move around {}: \
            \"Sources told me that {} is currently looking to swap {} with another team to change their dynamic. \
            It's not done yet, but there are chances for it to happen considering {}'s recent performances\".\n\n\
            We could try to get more info if you are interested.", host_name, player_name, team_short_name, player_name, team_short_name),
            "be.msg.rumour.subject8",
            "be.msg.rumour.body8",
        ),
        _ => (
            format!("{} — {} crashed out during scrims", rumour_media_name, player_name),
            format!("{} gave more insight on the recent drama around {}: \
            \"As far as I know, {} did not exactly punch the head coach as you can read on X or Reddit but there was \
            clearly a lot of animosity during scrims. The coaching staff even made a sub play for the latest scrims. \
            It is unsure if {} will be able to play for the next matches. Behavior issues like that can fade quickly \
            or completely break a team. I'll stay alert and tell you when I have more information\".\n\n\
            {} apparently has behavior issues. Keep that in mind in case you were interested by this player \
            for the upcoming off-season", host_name, team_short_name, player_name, player_name, player_name),
            "be.msg.rumour.subject9",
            "be.msg.rumour.body9",
        ),
    };

    let msg = InboxMessage::new(
        msg_id.to_string(),
        subject,
        body,
        "Media team".to_string(),
        date.to_string(),
    )
    .with_category(MessageCategory::Media)
    .with_priority(MessagePriority::Normal)
    .with_sender_role("Lead Communication")
    .with_action(action(
        "ack",
        "Noted",
        "be.msg.event.ack",
        ActionType::Acknowledge,
    ))
    .with_i18n(
        subject_key,
        body_key,
        params(&[
            ("team", team_name),
            ("team_short", team_short_name),
            ("player", player_name),
            ("rumour", rumour_media_name),
            ("host", host_name)]),
    )
    .with_sender_i18n("be.sender.media", "be.role.leadCom")
    .with_context(MessageContext {
        player_id: Some(player.id.clone()),
        ..Default::default()
    });
    msg
}


pub(super) fn al_lio_rumour_message(
    msg_id: &str,
    team: &Team,
    player: &Player,
    date: &str,
) -> Option<InboxMessage> {
    let rumour_media_name = "Al Lío Podcast";
    let host_names = vec!["Eros"];

    // Add special messages here if relevant to create more engaging stories
    // Use randomness to select them or a default_stream_message
    return Some(default_rumour_message(
        msg_id, 
        rumour_media_name,
        host_names,
        team,
        player,
        date));
}

pub(super) fn sheep_esports_rumour_message(
    msg_id: &str,
    team: &Team,
    player: &Player,
    date: &str,
) -> Option<InboxMessage> {
    let rumour_media_name = "Sheep Esports";
    let host_names = vec!["Wooloo", "Anonimotum"];

    // Add special messages here if relevant to create more engaging stories
    // Use randomness to select them or a default_stream_message
    return Some(default_rumour_message(
        msg_id, 
        rumour_media_name,
        host_names,
        team,
        player,
        date));
}

pub(super) fn duyum_tcl_rumour_message(
    msg_id: &str,
    team: &Team,
    player: &Player,
    date: &str,
) -> Option<InboxMessage> {
    let rumour_media_name = "Duyum TCL";
    let host_names = vec!["Duyum"];

    // Add special messages here if relevant to create more engaging stories
    // Use randomness to select them or a default_stream_message

    // Duyum relays news mostly about turkish stuff
    if player.nationality != "TR" { return None }
    //Default case
    return Some(default_rumour_message(
        msg_id, 
        rumour_media_name,
        host_names,
        team,
        player,
        date));
}

pub fn stream_message(
    msg_id: &str, 
    is_positive: bool,
    team: &Team,
    manager: &Manager,
    player: &Player, 
    date: &str
) -> InboxMessage {
    // Each language is not a complete bubble as everyone can hear news from
    // outside of their language but in minority (with an english exception)
    // Weights can be ajusted independantly for relation between language bubbles
    // and within language bubbles (see *your_language*_streamer_message)
    let weights;
    let manager_lang = &manager.nationality;
    match manager_lang.as_str() {
        "DE" => weights = [75, 20, 0, 0, 0, 0, 0, 0, 5], // Weights estimated by non-german person
        "EN" => weights = [0, 85, 5, 5, 0, 0, 0, 0, 5],  // Weights estimated by non-english person
        "ES" => weights = [0, 20, 70, 5, 0, 0, 0, 0, 5], // Weights estimated by non-spanish person
        "FR" => weights = [0, 20, 5, 70, 0, 0, 0, 0, 5], // Weights estimated by french person (Vincent-LAMBERT)
        "PT" => weights = [0, 20, 0, 0, 75, 0, 0, 0, 5], // Weights estimated by non-portuguese person
        "BR" => weights = [0, 20, 0, 0, 0, 75, 0, 0, 5], // Weights estimated by non-brazilian person
        "TR" => weights = [0, 20, 0, 0, 0, 0, 75, 0, 5], // Weights estimated by non-turkish person
        "IT" => weights = [0, 20, 0, 0, 0, 0, 0, 75, 5], // Weights estimated by non-italian person
        _    => weights = [0, 85, 0, 0, 0, 0, 0, 0, 15], // Default settings lean towards english
    }
    let dist = WeightedIndex::new(&weights).unwrap();
    let mut rng = rand::rng();
    let selected_index = dist.sample(&mut rng);

    match selected_index {
        0 => return german_streamer_message(&msg_id, 
                                            is_positive, 
                                            team,
                                            player, 
                                            date),
        1 => return english_streamer_message(&msg_id, 
                                            is_positive, 
                                            team,
                                            player, 
                                            date),
        2 => return spanish_streamer_message(&msg_id, 
                                            is_positive, 
                                            team,
                                            player, 
                                            date),
        3 => return french_streamer_message(&msg_id, 
                                            is_positive, 
                                            team,
                                            player, 
                                            date),
        4 => return portuguese_streamer_message(&msg_id, 
                                            is_positive, 
                                            team,
                                            player, 
                                            date),
        5 => return brazilian_streamer_message(&msg_id, 
                                            is_positive, 
                                            team,
                                            player, 
                                            date),
        6 => return turkish_streamer_message(&msg_id, 
                                            is_positive, 
                                            team,
                                            player, 
                                            date),
        7 => return italian_streamer_message(&msg_id, 
                                            is_positive, 
                                            team,
                                            player, 
                                            date),  
        _ => unreachable!(),
    };
}

fn default_stream_message(
    msg_id: &str, 
    is_positive: bool,
    streamer_channel: &str,
    streamer_pseudo: &str,
    streamer_name: &str,
    team: &Team,
    player: &Player, 
    date: &str
) -> InboxMessage {
    let mut rng = rand::rng();
    let variant = rng.random_range(0..6usize);

    let player_name = &player.match_name;
    // let player_position = &player.position;
    let team_name = &team.name;
    let team_short_name = &team.short_name;

    let (subject, body, subject_key, body_key) = if is_positive {
        match variant {
            0 => (
                format!("{} — Even {} admits: viewership is up this week", streamer_channel, streamer_pseudo),
                format!("{} surprised with a rare positive take: \
                \"I'll be honest — the numbers are up this week and I'm not going to pretend otherwise. \
                When the matches are good, the audience comes. Simple\".\n\n\
                A positive moment for the whole scene.", streamer_pseudo),
                "be.msg.streamer.positive.subject0",
                "be.msg.streamer.positive.body0",
            ),
            1 => (
                format!("{} — {} praised {}", streamer_channel, streamer_pseudo, team_name),
                format!("{} emphasized how {} played good in the last match. The viewers particularly \
                enjoyed the in-depth explanation of the vision setup and the macro game.\n\n\
                This is the kind of visibility we like.", streamer_pseudo, team_short_name),
                "be.msg.streamer.positive.subject1",
                "be.msg.streamer.positive.body1",
            ),
            2 => (
                format!("{} — {} favorite player of {}", streamer_channel, player_name, streamer_pseudo),
                format!("{} cannot stop raving about {}: \
                \"I won't lie, I think that {} is a really good player. enjoyed the in-depth \
                explanation of the vision setup and the macro game\".\n\n\
                {} receives a lot of praises these days.", streamer_pseudo, player_name, player_name, player_name),
                "be.msg.streamer.positive.subject2",
                "be.msg.streamer.positive.body2",
            ),
            3 => (
                format!("{} — {} particularly fond of {}'s gameplay", streamer_channel, streamer_pseudo, team_short_name),
                format!("{} emphasized how {} played good in the last match: \
                \"{} is really good right now and among the best players of the league. \
                I agreed with the haters but truth has to be told, I was wrong about {}\".\n\n\
                {} rarely admits to being wrong.", streamer_pseudo, player_name, player_name, team_short_name, streamer_pseudo),
                "be.msg.streamer.positive.subject3",
                "be.msg.streamer.positive.body3",
            ),
            4 => (
                format!("{} — {} commended {} fans", streamer_channel, streamer_pseudo, team_short_name),
                format!("{} particularly praised {} fans in a recent stream: \
                \"This is the kind of atmosphere I want to see every day. Sane rivalry, good banter, \
                I cannot ask more. Props to you {} fans, you really made your club proud!\".\n\n\
                We are lucky to have such a good community.", streamer_pseudo, team_short_name, team_short_name),
                "be.msg.streamer.positive.subject4",
                "be.msg.streamer.positive.body4",
            ),
            _ => (
                format!("{} — This week's matches actually got {} excited", streamer_channel, streamer_pseudo),
                format!("Not something you hear every day, {} was enthusiastic on a recent stream: \
                \"This week delivered. Real matches, proper stakes, viewers who stayed till the end. \
                This is what the league can be when it tries\".\n\n\
                Good week to be in esports.", streamer_pseudo),
                "be.msg.streamer.positive.subject5",
                "be.msg.streamer.positive.body5",
            ),
        }
    } else {
        match variant {
            0 => (
                format!("{} — league viewers are tanking and nobody cares", streamer_channel),
                format!("{} opened the morning stream with a full rant: \
                \"The numbers don't lie. Viewership is going down week after week and the league keeps \
                doing the same things expecting different results. C'est fini if nothing changes\".\n\n\
                Just background noise — but when {} talks, the community listens.", streamer_name, streamer_pseudo),
                "be.msg.streamer.negative.subject0",
                "be.msg.streamer.negative.body0",
            ),
            1 => (
                format!("{} — {} strongly critized {}", streamer_channel, streamer_pseudo, player_name),
                format!("{} insisted on critizing {} in a recent stream: \
                \"How is it possible to be that bad? Seriously {}, look at your map! \
                I swear even in Bronze, people play better.\".\n\n\
                I went to discuss with {} in private to make sure excuses are made.", streamer_pseudo, player_name, player_name, streamer_pseudo),
                "be.msg.streamer.negative.subject1",
                "be.msg.streamer.negative.body1",
            ),
            2 => (
                format!("{} — {} is among the worst players of the league according to {}", streamer_channel, player_name, streamer_pseudo),
                format!("{} has been adamant about {}: \
                \"I will be brutal. {} is among the bottom 3 of the league across all position. \
                {} did all they could to make this team work but if {} has no hands, \
                they cannot do miracles\".\n\n\
                {} is borderline harassing {} at this point.", streamer_pseudo, player_name, player_name, team_short_name, player_name, streamer_pseudo, player_name),
                "be.msg.streamer.negative.subject2",
                "be.msg.streamer.negative.body2",
            ),
            3 => (
                format!("{} — {} particularly critical about {}", streamer_channel, streamer_pseudo, player_name),
                format!("{} has been hard on {}: \
                \"I won't lie. I don't think {} is good enough to compete at this level. \
                In tier 2, this kind of mistake is not punished but in this league, it's a big no. \
                I don't wish {} bad things you know but I think that there are other players in tier 2 \
                that are better and who should be given the chance to prove it\".\n\n\
                What are our options in tier 2?", streamer_pseudo, player_name, player_name, player_name),
                "be.msg.streamer.negative.subject3",
                "be.msg.streamer.negative.body3",
            ),
            4 => (
                format!("{} — {} started a drama with {} fans", streamer_channel, streamer_pseudo, team_short_name),
                format!("{} has targetted our fans: \
                \"You are animals really. Your community is so toxic it's unbelievable. And I am sure \
                your owner won't tell shit as always. Just fucking back off my stream and eat \
                shit, assholes\".\n\n\
                This behavior is absolutely unbearable. I sent a message to Riot. \
                Just make sure the players defend our community on the Rift.", streamer_pseudo),
                "be.msg.streamer.negative.subject4",
                "be.msg.streamer.negative.body4",
            ),
            _ => (
                format!("{} — The league product is broken, according to {}", streamer_channel, streamer_pseudo),
                format!("{} spent a solid thirty minutes analysing the league's declining \
                engagement on a recent stream: \
                \"I've been saying this for months. The product isn't good enough. \
                The format is boring and the scheduling kills momentum\".\n\n\
                Harsh, but {}'s audience takes it seriously.", streamer_channel, streamer_pseudo),
                "be.msg.streamer.negative.subject5",
                "be.msg.streamer.negative.body5",
            ),
        }
    };
    
    let msg = InboxMessage::new(
        msg_id.to_string(),
        subject,
        body,
        "Media team".to_string(),
        date.to_string(),
    )
    .with_category(MessageCategory::Media)
    .with_priority(MessagePriority::Low)
    .with_sender_role("Lead Communication")
    .with_action(action(
        "ack",
        "Noted",
        "be.msg.event.ack",
        ActionType::Acknowledge,
    ))
    .with_i18n(
        &subject_key, 
        &body_key, 
        params(&[
            ("team", team_name),
            ("team_short", team_short_name),
            ("player", player_name),
            ("channel", streamer_channel), 
            ("pseudo", streamer_pseudo), 
            ("name", streamer_name)]))
    .with_sender_i18n("be.sender.media", "be.role.leadCom");
    msg
}

pub(super) fn tolkin_stream_message(
    msg_id: &str, 
    is_positive: bool, 
    team: &Team,
    player: &Player, 
    date: &str
) -> Option<InboxMessage> {
    let streamer_channel = "tolkin";
    let streamer_pseudo = "Tolkin";
    let streamer_name = "Niklot Stüber";

    // Add special messages here if relevant to create more engaging stories
    // Use randomness to select them or a default_stream_message
    return Some(default_stream_message(
        msg_id, 
        is_positive, 
        streamer_channel,
        streamer_pseudo, 
        streamer_name,
        team,
        player,
        date));
}

pub(super) fn obsess_stream_message(
    msg_id: &str, 
    is_positive: bool, 
    team: &Team,
    player: &Player, 
    date: &str
) -> Option<InboxMessage> {
    let streamer_channel = "obsess3";
    let streamer_pseudo = "Obsess";
    let streamer_name = "Patrick Engelmann";

    // Add special messages here if relevant to create more engaging stories
    // Use randomness to select them or a default_stream_message

    // This streamer is affiliated with a team and won't say negative things on it
    if team.short_name == "FNC" && !is_positive { return None }
    // Default case
    return Some(default_stream_message(
        msg_id, 
        is_positive, 
        streamer_channel,
        streamer_pseudo, 
        streamer_name,
        team,
        player,
        date));
}

pub(super) fn karni_stream_message(
    msg_id: &str, 
    is_positive: bool, 
    team: &Team,
    player: &Player, 
    date: &str
) -> Option<InboxMessage> {
    let streamer_channel = "karni";
    let streamer_pseudo = "Karni";
    let streamer_name = "Lukas Steininger";

    // Add special messages here if relevant to create more engaging stories
    // Use randomness to select them or a default_stream_message

    // This streamer is affiliated with a team and won't say negative things on it
    if team.short_name == "SK" && !is_positive { return None }
    // Default case
    return Some(default_stream_message(
        msg_id, 
        is_positive, 
        streamer_channel,
        streamer_pseudo, 
        streamer_name,
        team,
        player,
        date));
}

pub(super) fn sola_stream_message(
    msg_id: &str, 
    is_positive: bool, 
    team: &Team,
    player: &Player, 
    date: &str
) -> Option<InboxMessage> {
    let streamer_channel = "sola";
    let streamer_pseudo = "Sola";
    let streamer_name = "Nico Linke";

    // Add special messages here if relevant to create more engaging stories
    // Use randomness to select them or a default_stream_message

    // This streamer is affiliated with a team and won't say negative things on it
    if team.short_name == "G2" && !is_positive { return None }
    // Default case
    return Some(default_stream_message(
        msg_id, 
        is_positive, 
        streamer_channel,
        streamer_pseudo, 
        streamer_name,
        team,
        player,
        date));
}

pub(super) fn caedrel_stream_message(
    msg_id: &str, 
    is_positive: bool, 
    team: &Team,
    player: &Player, 
    date: &str
) -> Option<InboxMessage> {
    let streamer_channel = "caedrel";
    let streamer_pseudo = "Caedrel";
    let streamer_name = "Marc Robert Lamont";

    // Add special messages here if relevant to create more engaging stories
    // Use randomness to select them or a default_stream_message
    return Some(default_stream_message(
        msg_id, 
        is_positive, 
        streamer_channel,
        streamer_pseudo, 
        streamer_name,
        team,
        player,
        date));
}

pub(super) fn jankos_stream_message(
    msg_id: &str, 
    is_positive: bool, 
    team: &Team,
    player: &Player, 
    date: &str
) -> Option<InboxMessage> {
    let streamer_channel = "jankos";
    let streamer_pseudo = "Jankos";
    let streamer_name = "Marcin Jankowski";

    // Add special messages here if relevant to create more engaging stories
    // Use randomness to select them or a default_stream_message

    // This streamer is affiliated with a team and won't say negative things on it
    if team.short_name == "G2" && !is_positive { return None }
    // Default case
    return Some(default_stream_message(
        msg_id, 
        is_positive, 
        streamer_channel,
        streamer_pseudo, 
        streamer_name,
        team,
        player,
        date));
}

pub(super) fn gtroubleinc_stream_message(
    msg_id: &str, 
    is_positive: bool, 
    team: &Team,
    player: &Player, 
    date: &str
) -> Option<InboxMessage> {
    let streamer_channel = "gtroubleinc";
    let streamer_pseudo = "Troubleinc";
    let streamer_name = "Georgia Paras";

    // Add special messages here if relevant to create more engaging stories
    // Use randomness to select them or a default_stream_message

    // This streamer is affiliated with a team and won't say negative things on it
    if team.short_name == "VIT" && !is_positive { return None }
    // Default case
    return Some(default_stream_message(
        msg_id, 
        is_positive, 
        streamer_channel,
        streamer_pseudo, 
        streamer_name,
        team,
        player,
        date));
}

pub(super) fn caltys_stream_message(
    msg_id: &str, 
    is_positive: bool, 
    team: &Team,
    player: &Player, 
    date: &str
) -> Option<InboxMessage> {
    let streamer_channel = "caltys";
    let streamer_pseudo = "Caltys";
    let streamer_name = "Maya Henckel";

    // Add special messages here if relevant to create more engaging stories
    // Use randomness to select them or a default_stream_message

    // This streamer is affiliated with a team and won't say negative things on it
    if team.short_name == "G2" && !is_positive { return None }
    // Default case
    return Some(default_stream_message(
        msg_id, 
        is_positive, 
        streamer_channel,
        streamer_pseudo, 
        streamer_name,
        team,
        player,
        date));
}

pub(super) fn ibai_stream_message(
    msg_id: &str, 
    is_positive: bool, 
    team: &Team,
    player: &Player, 
    date: &str
) -> Option<InboxMessage> {
    let streamer_channel = "ibai";
    let streamer_pseudo = "Ibai";
    let streamer_name = "Ibai Llanos Garatea";

    // Add special messages here if relevant to create more engaging stories
    // Use randomness to select them or a default_stream_message

    // This streamer is affiliated with a team and won't say negative things on it
    if team.short_name == "KOI" && !is_positive { return None }
    // Default case
    return Some(default_stream_message(
        msg_id, 
        is_positive, 
        streamer_channel,
        streamer_pseudo, 
        streamer_name,
        team,
        player,
        date));
}

pub(super) fn skain_streamer_message(
    msg_id: &str, 
    is_positive: bool, 
    team: &Team,
    player: &Player, 
    date: &str
) -> Option<InboxMessage> {
    let streamer_channel = "skain";
    let streamer_pseudo = "Skain";
    let streamer_name = "David Carbó Ferrer";

    // Add special messages here if relevant to create more engaging stories
    // Use randomness to select them or a default_stream_message
    return Some(default_stream_message(
        msg_id, 
        is_positive, 
        streamer_channel,
        streamer_pseudo, 
        streamer_name,
        team,
        player,
        date));
}

pub(super) fn werlyb_stream_message(
    msg_id: &str, 
    is_positive: bool, 
    team: &Team,
    player: &Player, 
    date: &str
) -> Option<InboxMessage> {
    let streamer_channel = "werlyb";
    let streamer_pseudo = "Werlyb";
    let streamer_name = "Jorge Casanovas Moreno-Torres";

    // Add special messages here if relevant to create more engaging stories
    // Use randomness to select them or a default_stream_message

    // This streamer is affiliated with a team and won't say negative things on it
    if team.short_name == "TH" && !is_positive { return None }
    // Default case
    return Some(default_stream_message(
        msg_id, 
        is_positive, 
        streamer_channel,
        streamer_pseudo, 
        streamer_name,
        team,
        player,
        date));
}

pub(super) fn getflakked_stream_message(
    msg_id: &str, 
    is_positive: bool, 
    team: &Team,
    player: &Player, 
    date: &str
) -> Option<InboxMessage> {
    let streamer_channel = "getflakked";
    let streamer_pseudo = "Flakked";
    let streamer_name = "Víctor Lirola Tortosa";

    // Add special messages here if relevant to create more engaging stories
    // Use randomness to select them or a default_stream_message

    // This streamer is affiliated with a team and won't say negative things on it
    if team.short_name == "GX" && !is_positive { return None }
    // Default case
    return Some(default_stream_message(
        msg_id, 
        is_positive, 
        streamer_channel,
        streamer_pseudo, 
        streamer_name,
        team,
        player,
        date));
}

pub(super) fn el_yuste_stream_message(
    msg_id: &str, 
    is_positive: bool, 
    team: &Team,
    player: &Player, 
    date: &str
) -> Option<InboxMessage> {
    let streamer_channel = "el_yuste";
    let streamer_pseudo = "Yuste";
    let streamer_name = "Antonio Yuste";

    // Add special messages here if relevant to create more engaging stories
    // Use randomness to select them or a default_stream_message
    return Some(default_stream_message(
        msg_id, 
        is_positive, 
        streamer_channel,
        streamer_pseudo, 
        streamer_name,
        team,
        player,
        date));
}

pub(super) fn kameto_stream_message(
    msg_id: &str, 
    is_positive: bool, 
    team: &Team,
    player: &Player, 
    date: &str
) -> Option<InboxMessage> {
    let streamer_channel = "kamet0";
    let streamer_pseudo = "Kameto";
    let streamer_name = "Kamel Kebir";

    // Add special messages here if relevant to create more engaging stories
    // Use randomness to select them or a default_stream_message

    // This streamer is affiliated with a team and won't say negative things on it
    if team.short_name == "KC" && !is_positive { return None }
    // Default case
    return Some(default_stream_message(
        msg_id, 
        is_positive, 
        streamer_channel,
        streamer_pseudo, 
        streamer_name,
        team,
        player,
        date));
}

pub(super) fn trayton_stream_message(
    msg_id: &str, 
    is_positive: bool, 
    team: &Team,
    player: &Player, 
    date: &str
) -> Option<InboxMessage> {
    let streamer_channel = "trayton";
    let streamer_pseudo = "Trayton";
    let streamer_name = "Jean Medzadourian";

    // Add special messages here if relevant to create more engaging stories
    // Use randomness to select them or a default_stream_message

    // This streamer is affiliated with a team and won't say negative things on it
    if team.short_name == "G2" && !is_positive { return None }
    // Default case
    return Some(default_stream_message(
        msg_id, 
        is_positive, 
        streamer_channel,
        streamer_pseudo, 
        streamer_name,
        team,
        player,
        date));
}

pub(super) fn zaboutine_stream_message(
    msg_id: &str, 
    is_positive: bool, 
    team: &Team,
    player: &Player, 
    date: &str
) -> Option<InboxMessage> {
    let streamer_channel = "zaboutine";
    let streamer_pseudo = "Zaboutine";
    let streamer_name = "Thomas Si-Hassen";

    // Add special messages here if relevant to create more engaging stories
    // Use randomness to select them or a default_stream_message

    // This streamer is affiliated with a team (Shifters) but will say negative things on it
    return Some(default_stream_message(
        msg_id, 
        is_positive, 
        streamer_channel,
        streamer_pseudo, 
        streamer_name,
        team,
        player,
        date));
}

pub(super) fn skyyart_stream_message(
    msg_id: &str, 
    is_positive: bool, 
    team: &Team,
    player: &Player, 
    date: &str
) -> Option<InboxMessage> {
    let streamer_channel = "skyyart";
    let streamer_pseudo = "Skyyart";
    let streamer_name = "Willy Dias";

    // Add special messages here if relevant to create more engaging stories
    // Use randomness to select them or a default_stream_message

    // This streamer is affiliated with a team and won't say negative things on it
    if team.short_name == "G2" && !is_positive { return None }
    // Default case
    return Some(default_stream_message(
        msg_id, 
        is_positive, 
        streamer_channel,
        streamer_pseudo, 
        streamer_name,
        team,
        player,
        date));
}

pub(super) fn peaxy_stream_message(
    msg_id: &str, 
    is_positive: bool, 
    team: &Team,
    player: &Player, 
    date: &str
) -> Option<InboxMessage> {
    let streamer_channel = "peaxy";
    let streamer_pseudo = "Peaxy";
    let streamer_name = "Tamara Murcia Peaxy";

    // Add special messages here if relevant to create more engaging stories
    // Use randomness to select them or a default_stream_message

    // This streamer is affiliated with a team and won't say negative things on it
    if team.short_name == "VIT" && !is_positive { return None }
    // Default case
    return Some(default_stream_message(
        msg_id, 
        is_positive, 
        streamer_channel,
        streamer_pseudo, 
        streamer_name,
        team,
        player,
        date));
}

pub(super) fn archarom_stream_message(
    msg_id: &str, 
    is_positive: bool, 
    team: &Team,
    player: &Player, 
    date: &str
) -> Option<InboxMessage> {
    let streamer_channel = "archarom";
    let streamer_pseudo = "Archarom";
    let streamer_name = "Alexandre Maia";

    // Add special messages here if relevant to create more engaging stories
    // Use randomness to select them or a default_stream_message
    return Some(default_stream_message(
        msg_id, 
        is_positive, 
        streamer_channel,
        streamer_pseudo, 
        streamer_name,
        team,
        player,
        date));
}

pub(super) fn kamus_stream_message(
    msg_id: &str, 
    is_positive: bool, 
    team: &Team,
    player: &Player, 
    date: &str
) -> Option<InboxMessage> {
    let streamer_channel = "kamuslol";
    let streamer_pseudo = "Kamus";
    let streamer_name = "Mikaël Carvalho";

    // Add special messages here if relevant to create more engaging stories
    // Use randomness to select them or a default_stream_message
    return Some(default_stream_message(
        msg_id, 
        is_positive, 
        streamer_channel,
        streamer_pseudo, 
        streamer_name,
        team,
        player,
        date));
}

pub(super) fn lynxcerezlol_stream_message(
    msg_id: &str, 
    is_positive: bool, 
    team: &Team,
    player: &Player, 
    date: &str
) -> Option<InboxMessage> {
    let streamer_channel = "lynxcerezlol";
    let streamer_pseudo = "Lynx"; // Previously Lynx Çerezz according to https://lol.fandom.com/wiki/Lynx_(Furkan_Ar%C4%B1kovan)
    let streamer_name = "Furkan Arıkovan";

    // Add special messages here if relevant to create more engaging stories
    // Use randomness to select them or a default_stream_message

    // This streamer is affiliated with a team and won't say negative things on it
    if team.short_name == "NAVI" && !is_positive { return None }
    // Default case
    return Some(default_stream_message(
        msg_id, 
        is_positive, 
        streamer_channel,
        streamer_pseudo, 
        streamer_name,
        team,
        player,
        date));
}

pub(super) fn halpern_stream_message(
    msg_id: &str, 
    is_positive: bool, 
    team: &Team,
    player: &Player, 
    date: &str
) -> Option<InboxMessage> {
    let streamer_channel = "halpern";
    let streamer_pseudo = "Halpern";
    let streamer_name = "Aral Norman";

    // Add special messages here if relevant to create more engaging stories
    // Use randomness to select them or a default_stream_message
    return Some(default_stream_message(
        msg_id, 
        is_positive, 
        streamer_channel,
        streamer_pseudo, 
        streamer_name,
        team,
        player,
        date));
}

pub(super) fn brizz94_stream_message(
    msg_id: &str, 
    is_positive: bool, 
    team: &Team,
    player: &Player, 
    date: &str
) -> Option<InboxMessage> {
    let streamer_channel = "brizz94";
    let streamer_pseudo = "Brizz";
    let streamer_name = "Luca Brizzante";

    // Add special messages here if relevant to create more engaging stories
    // Use randomness to select them or a default_stream_message

    // This streamer is affiliated with a team and won't say negative things on it
    if team.short_name == "G2" && !is_positive { return None }
    // Default case
    return Some(default_stream_message(
        msg_id, 
        is_positive, 
        streamer_channel,
        streamer_pseudo, 
        streamer_name,
        team,
        player,
        date));
}

pub(super) fn terenas_stream_message(
    msg_id: &str, 
    is_positive: bool, 
    team: &Team,
    player: &Player, 
    date: &str
) -> Option<InboxMessage> {
    let streamer_channel = "terenas";
    let streamer_pseudo = "Terenas";
    let streamer_name = "Lapo Raspanti";

    // Add special messages here if relevant to create more engaging stories
    // Use randomness to select them or a default_stream_message
    return Some(default_stream_message(
        msg_id, 
        is_positive, 
        streamer_channel,
        streamer_pseudo, 
        streamer_name,
        team,
        player,
        date));
}