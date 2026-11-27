# Media Calendar: Glossary and Data Model (v1)

Nov 27, 2026 · @Frederick Price

## Summary

The app runs one company's video production. Each company gets its own installation and database, so nothing can leak between companies. The company has one or more **content calendars**; each calendar holds **channels** and series; every video belongs to a **series**, breaks into **segments**, and tracks its **assets** until it is **published** on one or more platforms. People sit in **groups**, and each group gets a **role** on a whole calendar or on one channel, which sets edit, read or none for each type of object from there down. Every change and sign-in is written to an **audit log**. Each video is planned backwards from its finish date, so moving that date moves everything tied to it. Times are stored in UTC and shown in each person's own time zone. That is 18 content tables (8 of them for templates), 7 access tables, 9 small lookup tables and 1 audit table beside the existing `users` table.

How to read this draft:

- **(guess)** marks a choice I made without your input. Change it freely.
- **Q1, Q2, ...** point to the Open questions at the end.
- Nothing in the repo has changed. Migrations come only after you approve this model.

## Glossary

These are the terms the app will use in its screens and its code. The last column says where each term lives in the schema below, or that v1 leaves it out.

### Planning

| Term | Meaning | In the app |
| --- | --- | --- |
| Content calendar | The dated plan of what gets made and published, and when. Also called an editorial calendar. | calendars: the top of the access tree, shown as a month/week view of publish dates and milestones |
| Channel | One destination account on one platform, such as a YouTube channel or a Facebook Page. | `channels` |
| Platform | The service a channel lives on: YouTube, Facebook, and later others. | `channels.platform` |
| Series | A named, recurring run of videos with a shared format, such as a weekly show. | `series` |
| Cadence | How often a series releases, such as weekly or monthly. Used to lay out slots. | `series.cadence` |
| Slot | A planned episode created ahead of time from the cadence, holding its place on the calendar until its idea is filled in. A slot is an ordinary video in the Pitch stage, with a finish date that can be moved. | `videos` |
| Video | One finished piece of content. An *episode* is a video in a series. Every video belongs to a series; loose one-off videos go in a catch-all series such as "One-offs". | `videos` |
| Format | The shape of a video: long-form, Short/Reel, or live stream. | `videos.format` |
| Pitch / brief | A short statement of the idea, audience and angle, written before any work starts. A pitch is not a separate kind of thing: it is a video in its first state, Pitch, and green-lighting it moves the same video on. | `videos.brief` |
| Milestone | A checkpoint on the way to finishing, set as a time before the video's finish date (T-minus), such as script done 10 days before, shoot day, picture lock. It moves when the finish date moves. | `milestones` |
| Finish date (T-0) | The moment a video must be finished and ready to publish. Every video has one, even a pitch. Milestones and asset needs count back from it (T-minus) and go-live times count forward (T-plus), so moving it moves the whole plan. | `videos.finish_at` |
| Template | A reusable starting point. A video template holds format, brief, script, a rundown of library segments, the assets it always uses, milestones timed before finish and publishing defaults per channel. Segment templates (a standard intro, a sponsor read) and series templates work the same way. Saved from an existing item or built by hand; a template can never be published itself. | `video_templates` and its child tables |

### Structure of a video

| Term | Meaning | In the app |
| --- | --- | --- |
| Script | The written words and directions for the whole video. | `videos.script`, plus per-segment text |
| Rundown | The ordered list of a video's segments with their running times. | The segment list of a video, ordered by `position` |
| Segment | A self-contained part of a video in the rundown, such as the intro or one topic. | `segments` |
| Cold open / hook | The first seconds, before the intro, meant to stop viewers leaving. | `segments.kind = cold_open` |
| Intro / outro | The branded opening and closing segments. | `segments.kind` |
| Sponsor read | A paid segment for an advertiser. | `segments.kind = sponsor` |
| CTA | Call to action: what you ask viewers to do (subscribe, click a link). | `segments.kind = cta` |
| End screen | The last 5 to 20 seconds with clickable cards on YouTube. | `segments.kind = end_screen` |
| Scene / shot | Smaller units inside a segment: a scene is one location or moment; a shot is one continuous camera take. | Not in v1 (Q4) |
| Running time | How long a segment or video lasts. | `segments.planned_seconds` |

### Assets

| Term | Meaning | In the app |
| --- | --- | --- |
| Asset | Any media file or item a video needs before it can publish. Assets sit in the calendar's asset library, so one asset (a logo, intro music) can be used by many videos and segments through video\_assets and segment\_assets. | `assets` |
| A-roll | The main footage: the presenter or interview on camera. | `assets.kind = a_roll` |
| B-roll | Supporting footage cut over the A-roll. | `assets.kind = b_roll` |
| Stock footage | B-roll licensed from a library. | `assets.kind = stock` |
| Graphic / motion graphic | Still or animated artwork: titles, charts, overlays. | `assets.kind = graphic` |
| Lower third | The name or caption bar in the lower third of the frame. | `assets.kind = lower_third` |
| Music bed | Background music. | `assets.kind = music` |
| SFX | Sound effects. | `assets.kind = sfx` |
| VO | Voice-over: narration recorded apart from the picture. | `assets.kind = voiceover` |
| Thumbnail | The still image shown before someone clicks. | `assets.kind = thumbnail` |
| Captions / subtitles | Timed text of the dialogue (captions) or a translation (subtitles), usually an .srt file. | `assets.kind = captions` |
| Rights / license | Permission to use an asset you did not create, with its terms and expiry. | License columns on `assets` (guess, Q5) |

### Production stages

| Term | Meaning | In the app |
| --- | --- | --- |
| Pre-production | Everything before recording: brief, research, script, booking guests, gathering assets. | `videos.stage` |
| Production | Recording the A-roll, B-roll and VO. | `videos.stage` |
| Post-production | Editing, graphics, sound mix, captions. | `videos.stage` |
| Rough cut | The first assembled edit, in order, unpolished. | A milestone |
| Fine cut | The edit after review notes, close to final. | A milestone |
| Picture lock | The point where the edit's timing is frozen; only sound, colour and graphics change after it. | A milestone |
| QC | Quality control: a final check of picture, sound, captions and metadata before release. | `videos.stage = review` |
| Approval | The sign-off that a video may publish. | `videos.stage = ready` |

### Publishing

| Term | Meaning | In the app |
| --- | --- | --- |
| Publication | One release of a video to one channel, with its own title, description and date. A new video gets one per channel on its calendar; channels it will not go to are marked skipped. | `publications` |
| Metadata | The text that ships with a video: title, description, tags, category. | Columns on `publications` |
| Visibility | Who can see it: public, unlisted (link only) or private. | `publications.visibility` |
| Scheduled | Set to go live at a future time. | `publications.status = scheduled` |
| Premiere | A YouTube release that plays at a set time with live chat. | `publications.is_premiere` |
| Go-live / publish date | The moment a publication becomes visible. Planned as a time after the video's finish, so it moves with it; published\_at records when it really went live. | `videos.finish_at + publications.go_live_after`, `published_at` |

### Access and sharing

| Term | Meaning | In the app |
| --- | --- | --- |
| Instance | One installation of the app for one company, with its own database. A second company gets its own instance, so a breach or bug cannot cross between them. | settings (name, home time zone) |
| User | One person's time with the company, from a start date to an optional end date; someone who returns gets a new user. Each user has a time zone, used to show every time on screen. | `users`, `time_zones` |
| Admin | A user who creates calendars, channels, groups and roles and hands out access. Admins can edit everything. An auditor can read everything, including the full audit log, and change nothing. | `system_roles` |
| Group | A named set of people, such as Editors or Freelance designers. | `groups`, `group_members` |
| Object type | The kinds of thing access is set for: calendar, channel, series, video, segment, asset, publication, milestone, and audit\_log (who may read a calendar's history). | `object_type` enum |
| Access level | `none`, `read` or `edit`, in that order. | `access_level` enum |
| Role | A reusable set of levels, one per object type. Example: Thumbnail designer = edit assets, read videos and segments, none for publications. | `roles`, `role_permissions` |
| Grant | Gives one group one role on a whole calendar, or on one channel inside it, covering everything underneath. | `calendar_grants`, `channel_grants` |
| Effective access | What a person can actually do: the highest level from any of their groups' grants, or edit if they are an admin. | `effective_access` view |

## Workflow states

Each video has one **stage** that moves forward as work gets done; segments, assets and publications each have their own smaller status. Which moves are allowed is data, not code (see Steps between states below). The app shows a warning, not a block, when a video moves to Ready with assets still missing (guess).

&#91;embedded content: video stages · 7 stages, 1 loop back\]

Review can send a video back to post-production any number of times; only Ready videos get publications scheduled.

### One shared set of states

Videos, segments, assets and publications all follow the same plan, make, check, release pattern, so they share one `workflow_states` lookup table. A second small table, `workflow_state_usage`, says which states each type may use, in what order, and what to call them there. Each row stores the state's `pkid`; a composite key makes sure, for example, that a segment can only hold a state listed for segments. Every state has an `is_active` flag: switching one off hides it from new choices without breaking rows that already use it.

| Shared state | Video | Segment | Asset | Publication |
| --- | --- | --- | --- | --- |
| Pitch | Pitch (start) |  |  |  |
| Planned |  | Planned (start) | Needed (start) | Draft (start) |
| Pre-production | Pre-production | Scripted |  |  |
| Production | Production | Recorded |  |  |
| Post-production | Post-production | Edited |  |  |
| In progress |  |  | In progress |  |
| In review | Review |  |  |  |
| Ready | Ready |  | Ready |  |
| Approved |  | Final | Approved |  |
| Scheduled |  |  |  | Scheduled |
| Published | Published |  |  | Live |
| Not needed | Dropped | Cut | Not needed | Skipped |
| Removed |  |  |  | Removed |

Kinds and options that are not states use the same tiny-table pattern: `platforms`, `video_formats`, `segment_kinds`, `asset_kinds` and `visibilities` (public, unlisted, private), each with a code, a label, an order and `is_active`. Only `object_type` and `access_level` stay as built-in enums, because the access-check code is written against them and the order none, read, edit is what makes the comparison work.

### Steps between states

The workflow is data. `workflow_states` lists the states, `workflow_state_usage` says which ones each type uses, and `workflow_transitions` lists every allowed step with the button text people see ("Green-light", "Send back to edit", "Skip"). An admin can add a step, relabel it or switch it off without a code change. The screen shows only the steps allowed from where an item is now, and a small trigger makes the database refuse anything else: a new item must start in its type's first state, and each change must be a listed, active step. A pitch that is turned down moves to **Dropped**, so it stays on record and out of the way.

## Proposed Postgres schema

Thirty-five tables: eighteen for content (eight of them templates) with `calendars` at the top and `videos` at the centre, seven for access, nine small lookup tables, and one append-only audit log. No table names a company: each company has its own installation and database, and its name and home time zone are settings. The calendar is drawn from one stored plan date, `videos.finish_at`: milestones, asset needs and go-live times are offsets from it, worked out by the `calendar_entries` view.

**Design rule: the database must not be able to hold data that makes no sense.** Wherever possible the structure itself rules a bad state out: required links instead of optional ones, separate tables instead of a column that means "one of two things", composite foreign keys so related rows must share a calendar or company, and CHECK, UNIQUE and exclusion constraints for everything else. A trigger is used only when a rule truly cannot be written that way; today that is the audit log, the no-delete guard and the check that a state change follows the workflow table. App code and screens may get more complex to keep admins' lives easy, but they never replace a database rule. They only add friendly checks in front of it.

&#91;embedded content: tables and links · 7 tables\]

A publication joins a video to a channel, so one video can go out on YouTube and Facebook with its own title and date on each.

### Ownership, access and safety rules

- **Nothing with history is deleted.** Every table blocks `DELETE` twice: the app's database login lacks the right, and a trigger refuses it anyway. Only a database owner who deliberately switches the trigger off can remove a row. Because creating a row is itself logged, every row has audit history from its first moment, so "no deletes at all" is the same rule as "no deletes once audited", and simpler to build.
- **Archive instead.** People-facing tables have `one archived_at column; who archived a row, like who created or changed it, is only in the audit log`. Archiving is one click for anyone with `edit` on that object type, is logged, and can be undone. Archived rows vanish from normal pages; the audit view still shows them. Archiving never cascades: an archived video's segments and milestones stay as they were, simply unreachable from normal pages, and the assets it used stay in the calendar's asset library for other videos.
- **A user is one stretch of employment.** Users have `started_at` and an optional `ended_at`, both full timestamps. Someone who leaves and comes back gets a new user; the same login name may be reused because the two periods cannot overlap.
- **Roles are dated too.** Admin and auditor assignments, group memberships and grants each have `starts_at` and an optional `ends_at`. Dates can be edited, every edit is logged, and exclusion constraints refuse any change that would make two periods for the same thing overlap. No assignment or membership can start before the user's start date or end after their end date, and the database itself refuses it, so a coding mistake cannot open that hole. Each dated row carries a copy of the user's employment range; a composite foreign key with ON UPDATE CASCADE keeps the copy equal to the user's real dates, and a CHECK keeps the row's period inside it. Shortening a user's end date is refused while any of their roles still runs later, so the app ends those roles first in the same transaction. As a second guard, access is still intersected with the employment dates.
- **Native constraints, not triggers.** Every foreign key references its parent's `pkid`. Where two parents must agree, such as a publication's video and channel being on the same calendar, the child also carries the shared column and a composite foreign key enforces it against a `UNIQUE (pkid, calendar_id)` on the parent. Triggers are kept only for the audit log, the no-delete guard and the workflow-step check, which constraints cannot express.
- The access tree is: calendar, then series, then videos (every video is in a series), then segments, assets and milestones. Channels also sit under the calendar, and a publication joins a video to one of them. Each channel sits in exactly one calendar (guess, Q18).
- A video publishes to every channel on its calendar by default: creating it adds a draft publication for each channel. Leaving a channel out is normal and is recorded by setting that publication to `skipped`, with an optional reason.
- A grant gives a group a role either on a whole calendar (`calendar_grants`) or on one channel (`channel_grants`). They are separate tables, so no grant can be unclear about what it covers, and who manages a channel can change without touching the calendar's grants. A channel-only grant covers that channel and its publications (guess, Q12).
- `effective_access` lists every level a user holds and the period it is valid (`valid`). The question asked is always: does this user hold the needed level here, right now?
- When a person holds several levels, the highest wins; there is no explicit deny (guess, Q11).
- Admins have `edit` on everything and manage calendars, channels, groups, roles and grants. Auditors have `read` on everything, including the full audit log, and change nothing. A role with `read` on `audit_log` makes a calendar-only auditor (Q19).
- Postgres 15 or later is needed; the cloud test setup has 16. `btree_gist` provides the no-overlap constraints.
- States and kinds are lookup tables; the Rust code refers to them by their stable code (loaded once at startup), never by a pkid. Only object\_type and access\_level are Postgres enums, mapped with `#[derive(sqlx::Type)]` (guess). Every primary key is a column named pkid holding a meaningless id generated by the database (a UUID from gen\_random\_uuid(), or an identity number for the audit log), never built from table data. Duplicate records are stopped by UNIQUE and no-overlap constraints instead. This matches the existing `users` table.

&#91;embedded content: how access is granted · user, group, role, channel\]

The role is the only place per-type levels live, so changing what designers may do is one edit, not one per channel.

### Draft SQL

This is a sketch for review, not a migration. If you approve it, it would become `migrations/0002_content_calendar.sql`.

```sql
-- Conventions
--  * Every primary key is named pkid and generated by the database: pkid UUID DEFAULT gen_random_uuid().
--    (Lower case on purpose: Postgres folds unquoted PKID to pkid anyway, and this avoids quoting.)
--  * Every foreign key column is named <parent>_id and REFERENCES the parent's pkid.
--  * Where two parents must agree (same calendar, same video), the child also
--    carries that shared column and a composite foreign key enforces it, e.g.
--      FOREIGN KEY (channel_id, calendar_id) REFERENCES channels (pkid, calendar_id)
--    backed by UNIQUE (pkid, calendar_id) on the parent. Native constraints only.
--  * Rule one: the schema must not be able to store a state that makes no sense. Use the
--    structure first (required links, split tables instead of optional parents, composite
--    keys), then CHECK / UNIQUE / EXCLUDE. A trigger only when nothing else can express the
--    rule; today that is the audit log, the no-delete guard and the workflow-step check.
--  * One database per company. Nothing here names a company: everyone in this database
--    works for the same one, and a second company gets its own instance and database.
--    The company's name and home time zone are instance settings, not rows.
--  * Every date and time is TIMESTAMPTZ (stored as UTC) and shown in the viewer's own time
--    zone; there are no DATE columns. Periods are half-open [start, end): end is the first
--    moment the thing no longer applies, and NULL means open-ended.
--  * Work is planned backwards from each video's finish moment (videos.finish_at, T-0).
--    Milestones, asset needs and go-live times are stored as offsets from it, never as
--    their own dates, so moving finish_at moves everything and nothing can fall out of step.
--  * States and kinds live in small lookup tables; rows store the lookup's pkid.
--  * Who created, changed or archived a row is in the audit log, not repeated in the row.
--  * Duplicates are stopped by UNIQUE and no-overlap constraints, never by the key itself.

CREATE EXTENSION IF NOT EXISTS btree_gist;   -- lets one constraint mix '=' with time-range overlap

-- ===== System enums =====
-- Only these two stay enums: the access-check code is written against them, and
-- access_level's order (none < read < edit) is what makes level >= 'read' work.
CREATE TYPE object_type  AS ENUM ('calendar', 'channel', 'series', 'video', 'segment',
                                  'asset', 'publication', 'milestone', 'template', 'audit_log');
CREATE TYPE access_level AS ENUM ('none', 'read', 'edit');

-- ===== Lookup tables =====
-- Each holds a stable code the app refers to, the label people see, an order, and is_active.
-- An inactive value stays valid on rows that already use it but is no longer offered for new
-- choices; the app enforces that, since a foreign key cannot look at is_active.

-- One shared set of workflow states, reused by videos, segments, assets and publications.
CREATE TABLE workflow_states (
    pkid       UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    code       TEXT NOT NULL UNIQUE,       -- 'pitch', 'in_review', 'published', ...
    label      TEXT NOT NULL,
    sort_order INT  NOT NULL,
    is_active  BOOLEAN NOT NULL DEFAULT TRUE
);

-- Which states each type of object may use, in what order, and under what name.
CREATE TABLE workflow_state_usage (
    pkid        UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    state_id    UUID NOT NULL REFERENCES workflow_states(pkid),
    object_type object_type NOT NULL,
    label       TEXT,                     -- NULL = the state's own label; e.g. 'Skipped' for publications
    sort_order  INT  NOT NULL,
    is_initial  BOOLEAN NOT NULL DEFAULT FALSE,   -- where a new row starts
    done_before_finish INTERVAL CHECK (done_before_finish >= INTERVAL '0'),
                        -- T-minus: leave this state this long before the finish; NULL = no deadline
    is_active   BOOLEAN NOT NULL DEFAULT TRUE,
    UNIQUE (state_id, object_type),           -- target for the per-type foreign keys below
    UNIQUE (object_type, sort_order)
);
CREATE UNIQUE INDEX ON workflow_state_usage (object_type) WHERE is_initial;  -- one start per type

-- The workflow as data: each row is one allowed step from one state to another for one type.
-- Both ends must be states that type uses (composite keys); a step back, such as Review to
-- Post-production, is just another row. The trigger near the end refuses any other change.
CREATE TABLE workflow_transitions (
    pkid          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    object_type   object_type NOT NULL,
    from_state_id UUID NOT NULL,
    to_state_id   UUID NOT NULL,
    label         TEXT,                     -- button text, e.g. 'Send back to edit'
    sort_order    INT  NOT NULL,
    is_active     BOOLEAN NOT NULL DEFAULT TRUE,
    CHECK (from_state_id <> to_state_id),
    UNIQUE (object_type, from_state_id, to_state_id),
    FOREIGN KEY (from_state_id, object_type) REFERENCES workflow_state_usage (state_id, object_type),
    FOREIGN KEY (to_state_id,   object_type) REFERENCES workflow_state_usage (state_id, object_type)
);

CREATE TABLE platforms (
    pkid UUID PRIMARY KEY DEFAULT gen_random_uuid(), code TEXT NOT NULL UNIQUE,
    label TEXT NOT NULL, sort_order INT NOT NULL, is_active BOOLEAN NOT NULL DEFAULT TRUE);
CREATE TABLE video_formats (
    pkid UUID PRIMARY KEY DEFAULT gen_random_uuid(), code TEXT NOT NULL UNIQUE,
    label TEXT NOT NULL, sort_order INT NOT NULL, is_active BOOLEAN NOT NULL DEFAULT TRUE);
CREATE TABLE segment_kinds (
    pkid UUID PRIMARY KEY DEFAULT gen_random_uuid(), code TEXT NOT NULL UNIQUE,
    label TEXT NOT NULL, sort_order INT NOT NULL, is_active BOOLEAN NOT NULL DEFAULT TRUE);
CREATE TABLE asset_kinds (
    pkid UUID PRIMARY KEY DEFAULT gen_random_uuid(), code TEXT NOT NULL UNIQUE,
    label TEXT NOT NULL, sort_order INT NOT NULL, is_active BOOLEAN NOT NULL DEFAULT TRUE);
CREATE TABLE visibilities (
    pkid UUID PRIMARY KEY DEFAULT gen_random_uuid(), code TEXT NOT NULL UNIQUE,
    label TEXT NOT NULL, sort_order INT NOT NULL, is_active BOOLEAN NOT NULL DEFAULT TRUE);

-- Seed values
INSERT INTO workflow_states (code, label, sort_order) VALUES
    ('pitch', 'Pitch', 10),               ('planned', 'Planned', 20),
    ('pre_production', 'Pre-production', 30), ('production', 'Production', 40),
    ('post_production', 'Post-production', 50), ('in_progress', 'In progress', 60),
    ('in_review', 'In review', 70),       ('ready', 'Ready', 80),
    ('approved', 'Approved', 90),         ('scheduled', 'Scheduled', 100),
    ('published', 'Published', 110),      ('not_needed', 'Not needed', 120),
    ('removed', 'Removed', 130);

INSERT INTO workflow_state_usage (state_id, object_type, label, sort_order, is_initial)
SELECT s.pkid, u.object_type::object_type, u.label, u.sort_order, u.is_initial
  FROM (VALUES
    ('pitch',           'video',       NULL,        10, TRUE),
    ('pre_production',  'video',       NULL,        20, FALSE),
    ('production',      'video',       NULL,        30, FALSE),
    ('post_production', 'video',       NULL,        40, FALSE),
    ('in_review',       'video',       'Review',    50, FALSE),
    ('ready',           'video',       NULL,        60, FALSE),
    ('published',       'video',       NULL,        70, FALSE),
    ('not_needed',      'video',       'Dropped',   80, FALSE),
    ('planned',         'segment',     NULL,        10, TRUE),
    ('pre_production',  'segment',     'Scripted',  20, FALSE),
    ('production',      'segment',     'Recorded',  30, FALSE),
    ('post_production', 'segment',     'Edited',    40, FALSE),
    ('approved',        'segment',     'Final',     50, FALSE),
    ('not_needed',      'segment',     'Cut',       60, FALSE),
    ('planned',         'asset',       'Needed',    10, TRUE),
    ('in_progress',     'asset',       NULL,        20, FALSE),
    ('ready',           'asset',       NULL,        30, FALSE),
    ('approved',        'asset',       NULL,        40, FALSE),
    ('not_needed',      'asset',       NULL,        50, FALSE),
    ('planned',         'publication', 'Draft',     10, TRUE),
    ('scheduled',       'publication', NULL,        20, FALSE),
    ('published',       'publication', 'Live',      30, FALSE),
    ('not_needed',      'publication', 'Skipped',   40, FALSE),
    ('removed',         'publication', NULL,        50, FALSE)
  ) AS u(code, object_type, label, sort_order, is_initial)
  JOIN workflow_states s ON s.code = u.code;

-- First guess at the allowed steps; admins can add, relabel or switch them off later.
INSERT INTO workflow_transitions (object_type, from_state_id, to_state_id, label, sort_order)
SELECT t.object_type::object_type, f.pkid, d.pkid, t.label, t.sort_order
  FROM (VALUES
    ('video', 'pitch', 'pre_production', 'Green-light', 10),
    ('video', 'pitch', 'not_needed', 'Drop', 20),
    ('video', 'pre_production', 'production', NULL, 30),
    ('video', 'pre_production', 'not_needed', 'Drop', 40),
    ('video', 'production', 'post_production', NULL, 50),
    ('video', 'post_production', 'in_review', 'Send for review', 60),
    ('video', 'in_review', 'post_production', 'Send back to edit', 70),
    ('video', 'in_review', 'ready', 'Approve', 80),
    ('video', 'ready', 'published', NULL, 90),
    ('segment', 'planned', 'pre_production', NULL, 10),
    ('segment', 'pre_production', 'production', NULL, 20),
    ('segment', 'production', 'post_production', NULL, 30),
    ('segment', 'post_production', 'approved', NULL, 40),
    ('segment', 'approved', 'post_production', 'Re-edit', 50),
    ('segment', 'planned', 'not_needed', 'Cut', 60),
    ('segment', 'pre_production', 'not_needed', 'Cut', 70),
    ('segment', 'production', 'not_needed', 'Cut', 80),
    ('segment', 'post_production', 'not_needed', 'Cut', 90),
    ('asset', 'planned', 'in_progress', NULL, 10),
    ('asset', 'planned', 'ready', 'Already have it', 20),
    ('asset', 'in_progress', 'ready', NULL, 30),
    ('asset', 'ready', 'approved', 'Approve', 40),
    ('asset', 'ready', 'in_progress', 'Needs changes', 50),
    ('asset', 'planned', 'not_needed', NULL, 60),
    ('asset', 'in_progress', 'not_needed', NULL, 70),
    ('publication', 'planned', 'scheduled', NULL, 10),
    ('publication', 'scheduled', 'planned', 'Unschedule', 20),
    ('publication', 'scheduled', 'published', 'Went live', 30),
    ('publication', 'published', 'removed', 'Taken down', 40),
    ('publication', 'planned', 'not_needed', 'Skip', 50),
    ('publication', 'not_needed', 'planned', 'Un-skip', 60)
  ) AS t(object_type, from_code, to_code, label, sort_order)
  JOIN workflow_states f ON f.code = t.from_code
  JOIN workflow_states d ON d.code = t.to_code;

-- First guess at each video stage's deadline, counted back from the finish (all guesses).
UPDATE workflow_state_usage u SET done_before_finish = d.before::interval
  FROM (VALUES ('pitch', '28 days'), ('pre_production', '14 days'), ('production', '7 days'),
               ('post_production', '3 days'), ('in_review', '1 day'), ('ready', '0')
       ) AS d(code, before)
  JOIN workflow_states s ON s.code = d.code
 WHERE u.state_id = s.pkid AND u.object_type = 'video';

INSERT INTO platforms (code, label, sort_order) VALUES
    ('youtube', 'YouTube', 10), ('facebook', 'Facebook', 20), ('other', 'Other', 90);
INSERT INTO video_formats (code, label, sort_order) VALUES
    ('long_form', 'Long-form', 10), ('short', 'Short / Reel', 20), ('live', 'Live stream', 30);
INSERT INTO segment_kinds (code, label, sort_order) VALUES
    ('cold_open', 'Cold open', 10), ('intro', 'Intro', 20), ('main', 'Main', 30),
    ('sponsor', 'Sponsor read', 40), ('cta', 'Call to action', 50), ('outro', 'Outro', 60),
    ('end_screen', 'End screen', 70), ('other', 'Other', 90);
INSERT INTO asset_kinds (code, label, sort_order) VALUES
    ('a_roll', 'A-roll', 10), ('b_roll', 'B-roll', 20), ('stock', 'Stock footage', 30),
    ('graphic', 'Graphic', 40), ('lower_third', 'Lower third', 50), ('music', 'Music', 60),
    ('sfx', 'Sound effects', 70), ('voiceover', 'Voice-over', 80), ('thumbnail', 'Thumbnail', 90),
    ('captions', 'Captions', 100), ('other', 'Other', 190);
INSERT INTO visibilities (code, label, sort_order) VALUES
    ('public', 'Public', 10), ('unlisted', 'Unlisted', 20), ('private', 'Private', 30);

-- Time zones people can pick, so a user can never hold a misspelt or unknown zone.
-- code is the IANA name Postgres and Rust both understand, e.g. 'America/Toronto'.
CREATE TABLE time_zones (
    pkid      UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    code      TEXT NOT NULL UNIQUE,
    is_active BOOLEAN NOT NULL DEFAULT TRUE
);
INSERT INTO time_zones (code) SELECT name FROM pg_timezone_names;

-- ===== People =====
-- A user is one person's period with the company. Leaving sets ended_at (when access stops);
-- coming back later means a new user. A login name may be reused, but periods never overlap.
-- Only admins create users; there is no sign-up. See 'Accounts and sign-in' for each column.
ALTER TABLE users RENAME COLUMN id TO pkid;   -- existing table from 0001; app code updated to match
ALTER TABLE users
    ADD COLUMN login_name           TEXT,           -- assigned by an admin; filled, then set NOT NULL
    ADD COLUMN display_name         TEXT NOT NULL DEFAULT '',
    ADD COLUMN time_zone_id         UUID NOT NULL   -- times on screen are shown in this zone
        REFERENCES time_zones(pkid),                -- (0001 holds only test users; clear them first)
    ALTER COLUMN email              DROP NOT NULL,  -- notifications only; duplicates allowed
    ALTER COLUMN password_hash      DROP NOT NULL,  -- Argon2id PHC string (per-user salt inside)
    ADD COLUMN can_login            BOOLEAN NOT NULL DEFAULT TRUE,   -- FALSE = not a login account
    ADD COLUMN must_change_password BOOLEAN NOT NULL DEFAULT TRUE,
    ADD COLUMN password_changed_at  TIMESTAMPTZ,
    ADD COLUMN locked_at            TIMESTAMPTZ,    -- locked by an admin; stays until cleared
    ADD COLUMN locked_reason        TEXT,           -- who locked it is in the audit log
    ADD COLUMN failed_logins        INT NOT NULL DEFAULT 0,
    ADD COLUMN locked_until         TIMESTAMPTZ,    -- automatic lockout; clears itself
    ADD COLUMN last_login_at        TIMESTAMPTZ,
    ADD COLUMN totp_secret_enc      BYTEA,          -- two-factor secret, encrypted outside the DB (Q17)
    ADD COLUMN started_at           TIMESTAMPTZ NOT NULL DEFAULT now(),
    ADD COLUMN ended_at             TIMESTAMPTZ,    -- NULL = still with the company
    ADD COLUMN employment           TSTZRANGE       -- computed; copied into every dated row below
        GENERATED ALWAYS AS (tstzrange(started_at, ended_at, '[)')) STORED,
    ADD CONSTRAINT users_no_password_unless_login CHECK (can_login OR password_hash IS NULL),
    ADD CONSTRAINT users_dates_ok CHECK (ended_at IS NULL OR ended_at > started_at),
    ADD CONSTRAINT users_pkid_employment UNIQUE (pkid, employment),  -- target for 'inside employment' keys
    DROP CONSTRAINT users_email_key,
    ADD CONSTRAINT users_login_no_overlap EXCLUDE USING gist
        (lower(login_name) WITH =, tstzrange(started_at, ended_at, '[)') WITH &&);
DROP INDEX idx_users_email;                           -- email is no longer how people sign in

-- Inside employment, enforced by the database (no trigger):
--  * each dated row carries a copy of the person's employment range;
--  * a composite key forces that copy to equal users.employment, and ON UPDATE CASCADE
--    pushes any change to the person's dates down into the copy;
--  * a CHECK requires the row's own period to sit inside the copy.
-- So a role can never start before or run past employment. Shortening a person's end
-- date while a role still runs later is refused as a whole; the app ends those roles
-- first, in the same transaction. The access view also intersects every period with
-- employment, as a second guard.
CREATE TABLE system_roles (                   -- dated admin and auditor assignments, whole instance
    pkid       UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id    UUID NOT NULL REFERENCES users(pkid),
    role       TEXT NOT NULL CHECK (role IN ('admin', 'auditor')),
    starts_at  TIMESTAMPTZ NOT NULL,
    ends_at    TIMESTAMPTZ,                 -- NULL = open-ended
    employment TSTZRANGE NOT NULL,          -- copy of the person's employment
    CHECK (ends_at IS NULL OR ends_at > starts_at),
    FOREIGN KEY (user_id, employment) REFERENCES users (pkid, employment) ON UPDATE CASCADE,
    CHECK (employment @> tstzrange(starts_at, ends_at, '[)')),  -- inside employment
    EXCLUDE USING gist (user_id WITH =, role WITH =,
                        tstzrange(starts_at, ends_at, '[)') WITH &&)
);

CREATE TABLE groups (
    pkid         UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name         TEXT NOT NULL,
    archived_at  TIMESTAMPTZ
);
CREATE UNIQUE INDEX ON groups (lower(name)) WHERE archived_at IS NULL;

CREATE TABLE group_members (                  -- dated: who was in which group, when
    pkid         UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    group_id     UUID NOT NULL REFERENCES groups(pkid),
    user_id      UUID NOT NULL REFERENCES users(pkid),
    starts_at    TIMESTAMPTZ NOT NULL,
    ends_at      TIMESTAMPTZ,
    employment   TSTZRANGE NOT NULL,        -- copy of the person's employment
    CHECK (ends_at IS NULL OR ends_at > starts_at),
    FOREIGN KEY (user_id, employment) REFERENCES users (pkid, employment) ON UPDATE CASCADE,
    CHECK (employment @> tstzrange(starts_at, ends_at, '[)')),  -- inside employment
    EXCLUDE USING gist (group_id WITH =, user_id WITH =,
                        tstzrange(starts_at, ends_at, '[)') WITH &&)
);

CREATE TABLE roles (
    pkid         UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name         TEXT NOT NULL,             -- 'Editor', 'Viewer', 'Thumbnail designer', 'Auditor'
    archived_at  TIMESTAMPTZ
);
CREATE UNIQUE INDEX ON roles (lower(name)) WHERE archived_at IS NULL;

CREATE TABLE role_permissions (               -- one row per type; lowering access sets 'none'
    pkid        UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    role_id     UUID NOT NULL REFERENCES roles(pkid),
    object_type object_type  NOT NULL,
    level       access_level NOT NULL,
    UNIQUE (role_id, object_type)             -- no duplicate rows for a type
);

-- ===== Content: calendar at the top =====
-- Rows with a workflow state carry state_id plus a fixed state_scope column; the composite
-- key on the pair means a video can only hold a state that is listed for videos, and so on.
-- archived_at hides a row from normal pages. It never cascades: archiving a video leaves its
-- segments, milestones and publications as they are, just unreachable from normal pages.
CREATE TABLE calendars (
    pkid         UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name         TEXT NOT NULL,            -- e.g. 'Main brand', 'Client: Acme'
    description  TEXT,
    created_at   TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    archived_at  TIMESTAMPTZ
);
CREATE UNIQUE INDEX ON calendars (lower(name)) WHERE archived_at IS NULL;

CREATE TABLE channels (
    pkid         UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    calendar_id  UUID NOT NULL REFERENCES calendars(pkid),
    name         TEXT NOT NULL,
    platform_id  UUID NOT NULL REFERENCES platforms(pkid),
    url          TEXT,
    created_at   TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    archived_at  TIMESTAMPTZ,
    UNIQUE (pkid, calendar_id)
);
CREATE UNIQUE INDEX ON channels (calendar_id, lower(name)) WHERE archived_at IS NULL;  -- no twin channels

-- Two grant tables, so no row has an optional target: a grant is on a whole calendar or
-- on one channel, never 'maybe a channel'. Who manages a channel can change without
-- touching the calendar's grants.
CREATE TABLE calendar_grants (                -- dated: a group holds a role on a whole calendar
    pkid         UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    calendar_id  UUID NOT NULL REFERENCES calendars(pkid),
    group_id     UUID NOT NULL REFERENCES groups(pkid),
    role_id      UUID NOT NULL REFERENCES roles(pkid),
    starts_at    TIMESTAMPTZ NOT NULL,
    ends_at      TIMESTAMPTZ,
    CHECK (ends_at IS NULL OR ends_at > starts_at),
    EXCLUDE USING gist (group_id WITH =, calendar_id WITH =,
                        tstzrange(starts_at, ends_at, '[)') WITH &&)
);

CREATE TABLE channel_grants (                 -- dated: a group holds a role on one channel
    pkid         UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    calendar_id  UUID NOT NULL REFERENCES calendars(pkid),
    channel_id   UUID NOT NULL REFERENCES channels(pkid),
    group_id     UUID NOT NULL REFERENCES groups(pkid),
    role_id      UUID NOT NULL REFERENCES roles(pkid),
    starts_at    TIMESTAMPTZ NOT NULL,
    ends_at      TIMESTAMPTZ,
    CHECK (ends_at IS NULL OR ends_at > starts_at),
    FOREIGN KEY (channel_id, calendar_id)   REFERENCES channels  (pkid, calendar_id),   -- channel on that calendar
    EXCLUDE USING gist (group_id WITH =, channel_id WITH =,
                        tstzrange(starts_at, ends_at, '[)') WITH &&)
);

CREATE TABLE series (
    pkid         UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    calendar_id  UUID NOT NULL REFERENCES calendars(pkid),
    title        TEXT NOT NULL,
    description  TEXT,
    cadence      INTERVAL CHECK (cadence > INTERVAL '0'),  -- '7 days', '14 days', '1 month';
                                       -- NULL = no regular rhythm, e.g. 'One-offs'. Used only when
                                       -- the app adds slots; each slot is then a video row of its own
    created_at   TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    archived_at  TIMESTAMPTZ,
    UNIQUE (pkid, calendar_id)
);
CREATE UNIQUE INDEX ON series (calendar_id, lower(title)) WHERE archived_at IS NULL;   -- no twin series

CREATE TABLE videos (
    pkid              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    calendar_id       UUID NOT NULL REFERENCES calendars(pkid),  -- decides access; always the
                      -- series' calendar (key below), kept here so segments, assets and
                      -- publications can check 'same calendar' with one composite key
    series_id         UUID NOT NULL REFERENCES series(pkid),  -- every video is in a series;
                                        -- loose videos go in e.g. a 'One-offs' series
    episode_number    INT,
    working_title     TEXT NOT NULL,
    format_id         UUID NOT NULL REFERENCES video_formats(pkid),
    state_id          UUID NOT NULL REFERENCES workflow_states(pkid),
    state_scope       object_type NOT NULL DEFAULT 'video' CHECK (state_scope = 'video'),
    brief             TEXT,                -- the pitch; a pitch is a video in the 'Pitch' state
    script            TEXT,
    finish_at         TIMESTAMPTZ NOT NULL,  -- T-0: when it must be finished, ready to publish.
                                         -- For a pitch, when it would need to be finished.
    created_at        TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at        TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    archived_at       TIMESTAMPTZ,
    UNIQUE (pkid, calendar_id),
    FOREIGN KEY (series_id, calendar_id) REFERENCES series (pkid, calendar_id),  -- series on same calendar
    FOREIGN KEY (state_id, state_scope)  REFERENCES workflow_state_usage (state_id, object_type)
);
CREATE UNIQUE INDEX ON videos (series_id, episode_number)
    WHERE archived_at IS NULL AND episode_number IS NOT NULL;  -- no two episode 5s in a series

CREATE TABLE segments (
    pkid            UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    calendar_id     UUID NOT NULL REFERENCES calendars(pkid),  -- always the video's (key below)
    video_id        UUID NOT NULL REFERENCES videos(pkid),
    position        INT  NOT NULL,         -- order in the rundown
    kind_id         UUID NOT NULL REFERENCES segment_kinds(pkid),
    title           TEXT NOT NULL,
    script          TEXT,
    planned_seconds INT CHECK (planned_seconds >= 0),
    state_id        UUID NOT NULL REFERENCES workflow_states(pkid),
    state_scope     object_type NOT NULL DEFAULT 'segment' CHECK (state_scope = 'segment'),
    archived_at     TIMESTAMPTZ,
    UNIQUE (pkid, calendar_id),
    FOREIGN KEY (video_id, calendar_id) REFERENCES videos (pkid, calendar_id),
    FOREIGN KEY (state_id, state_scope) REFERENCES workflow_state_usage (state_id, object_type),
    -- positions are unique among live segments only; deferred so a reorder can swap rows
    EXCLUDE (video_id WITH =, position WITH =) WHERE (archived_at IS NULL)
        DEFERRABLE INITIALLY DEFERRED
);

-- Assets belong to the calendar, not to one video, so the same intro music or logo can be
-- used by many videos. Their state and rights describe the asset itself.
CREATE TABLE assets (
    pkid               UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    calendar_id        UUID NOT NULL REFERENCES calendars(pkid),
    kind_id            UUID NOT NULL REFERENCES asset_kinds(pkid),
    name               TEXT NOT NULL,
    state_id           UUID NOT NULL REFERENCES workflow_states(pkid),
    state_scope        object_type NOT NULL DEFAULT 'asset' CHECK (state_scope = 'asset'),
    location           TEXT,               -- URL or file path; the app stores no files (Q6)
                                           -- no due date here: each use says when it is needed
    notes              TEXT,
    license_source     TEXT,               -- rights, guess (Q5)
    license_terms      TEXT,
    license_expires_at TIMESTAMPTZ,
    created_at         TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    archived_at        TIMESTAMPTZ,
    UNIQUE (pkid, calendar_id),
    FOREIGN KEY (state_id, state_scope) REFERENCES workflow_state_usage (state_id, object_type)
);

-- Where an asset is used. Videos and segments pull assets in; neither is the asset's parent.
-- Two link tables, so no link has an optional segment: an asset is used either by the whole
-- video (thumbnail, captions) or by one segment (a B-roll clip, a music bed).
CREATE TABLE video_assets (
    pkid        UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    calendar_id UUID NOT NULL REFERENCES calendars(pkid),
    video_id    UUID NOT NULL REFERENCES videos(pkid),
    asset_id    UUID NOT NULL REFERENCES assets(pkid),
    notes       TEXT,                     -- e.g. 'YouTube thumbnail only'
    needed_before INTERVAL NOT NULL DEFAULT INTERVAL '0' CHECK (needed_before >= INTERVAL '0'),
                                          -- T-minus: needed this long before the video's finish
    archived_at TIMESTAMPTZ,              -- un-linking an asset from a video
    FOREIGN KEY (video_id, calendar_id) REFERENCES videos (pkid, calendar_id),     -- video and asset
    FOREIGN KEY (asset_id, calendar_id) REFERENCES assets (pkid, calendar_id)      -- share a calendar
);
CREATE UNIQUE INDEX ON video_assets (video_id, asset_id) WHERE archived_at IS NULL;      -- no double links

CREATE TABLE segment_assets (
    pkid        UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    calendar_id UUID NOT NULL REFERENCES calendars(pkid),
    segment_id  UUID NOT NULL REFERENCES segments(pkid),
    asset_id    UUID NOT NULL REFERENCES assets(pkid),
    notes       TEXT,                     -- e.g. 'only the first 10 seconds'
    needed_before INTERVAL NOT NULL DEFAULT INTERVAL '0' CHECK (needed_before >= INTERVAL '0'),
    archived_at TIMESTAMPTZ,
    FOREIGN KEY (segment_id, calendar_id) REFERENCES segments (pkid, calendar_id), -- segment and asset
    FOREIGN KEY (asset_id, calendar_id)   REFERENCES assets (pkid, calendar_id)    -- share a calendar
);
CREATE UNIQUE INDEX ON segment_assets (segment_id, asset_id) WHERE archived_at IS NULL;  -- no double links

-- One row per (video, channel). When a video is created, the app adds a 'Draft' row for
-- every channel on its calendar; leaving a channel out means moving that row to 'Skipped'.
CREATE TABLE publications (
    pkid          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    calendar_id   UUID NOT NULL REFERENCES calendars(pkid),
    video_id      UUID NOT NULL REFERENCES videos(pkid),
    channel_id    UUID NOT NULL REFERENCES channels(pkid),
    state_id      UUID NOT NULL REFERENCES workflow_states(pkid),
    state_scope   object_type NOT NULL DEFAULT 'publication' CHECK (state_scope = 'publication'),
    skip_reason   TEXT,                    -- optional note when skipped
    visibility_id UUID NOT NULL REFERENCES visibilities(pkid),
    is_premiere   BOOLEAN NOT NULL DEFAULT FALSE,
    title         TEXT,                    -- NULL = use the video's title
    description   TEXT,
    tags          TEXT[] NOT NULL DEFAULT '{}',
    go_live_after INTERVAL NOT NULL DEFAULT INTERVAL '0' CHECK (go_live_after >= INTERVAL '0'),
                                           -- T-plus: each channel goes live this long after the
                                           -- video's finish, so it moves when finish_at moves
    published_at  TIMESTAMPTZ,             -- what actually happened; fixed once set
    platform_url  TEXT,
    archived_at   TIMESTAMPTZ,
    UNIQUE (video_id, channel_id),          -- one publication per video per channel
    FOREIGN KEY (video_id, calendar_id)   REFERENCES videos   (pkid, calendar_id),   -- channel and video
    FOREIGN KEY (channel_id, calendar_id) REFERENCES channels (pkid, calendar_id),   -- share a calendar
    FOREIGN KEY (state_id, state_scope)   REFERENCES workflow_state_usage (state_id, object_type)
);

CREATE TABLE milestones (
    pkid         UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    video_id     UUID NOT NULL REFERENCES videos(pkid),
    name         TEXT NOT NULL,            -- 'Shoot day', 'Picture lock', ...
    before_finish INTERVAL NOT NULL CHECK (before_finish >= INTERVAL '0'),  -- T-minus; due moment
                                           -- = videos.finish_at - before_finish (see calendar_entries)
    done_at      TIMESTAMPTZ,              -- what actually happened; fixed once set
    archived_at  TIMESTAMPTZ
);
CREATE UNIQUE INDEX ON milestones (video_id, lower(name)) WHERE archived_at IS NULL;

-- ===== Templates =====
-- A template is a starting point, not real work: it has no workflow state, no publications and
-- no dates, only offsets, so it can never be scheduled or published by mistake. Separate tables
-- keep it that way. 'Save as template' copies a video's rundown, asset links, milestones and
-- publication settings in here; 'New video from template' copies them back out into a fresh
-- video. Nothing links the two afterwards (the audit log records which template was used).
CREATE TABLE video_templates (
    pkid         UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    calendar_id  UUID NOT NULL REFERENCES calendars(pkid),
    name         TEXT NOT NULL,              -- 'Weekly episode', 'Product review short'
    format_id    UUID NOT NULL REFERENCES video_formats(pkid),
    brief        TEXT,
    script       TEXT,
    created_at   TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    archived_at  TIMESTAMPTZ,
    UNIQUE (pkid, calendar_id)
);
CREATE UNIQUE INDEX ON video_templates (calendar_id, lower(name)) WHERE archived_at IS NULL;

-- A segment template stands on its own in the calendar's segment library ('Standard intro',
-- 'Sponsor read'), so it can be dropped into any video or used in any video template's rundown.
CREATE TABLE segment_templates (
    pkid            UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    calendar_id     UUID NOT NULL REFERENCES calendars(pkid),
    name            TEXT NOT NULL,
    kind_id         UUID NOT NULL REFERENCES segment_kinds(pkid),
    title           TEXT NOT NULL,           -- copied into the new segment
    script          TEXT,
    planned_seconds INT CHECK (planned_seconds >= 0),
    archived_at     TIMESTAMPTZ,
    UNIQUE (pkid, calendar_id)
);
CREATE UNIQUE INDEX ON segment_templates (calendar_id, lower(name)) WHERE archived_at IS NULL;

-- A video template's rundown: library segments in order.
CREATE TABLE video_template_segments (
    pkid                UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    calendar_id         UUID NOT NULL REFERENCES calendars(pkid),
    video_template_id   UUID NOT NULL REFERENCES video_templates(pkid),
    segment_template_id UUID NOT NULL REFERENCES segment_templates(pkid),
    position            INT  NOT NULL,
    archived_at         TIMESTAMPTZ,
    FOREIGN KEY (video_template_id, calendar_id)   REFERENCES video_templates (pkid, calendar_id),
    FOREIGN KEY (segment_template_id, calendar_id) REFERENCES segment_templates (pkid, calendar_id),
    EXCLUDE (video_template_id WITH =, position WITH =) WHERE (archived_at IS NULL)
        DEFERRABLE INITIALLY DEFERRED
);

CREATE TABLE video_template_assets (            -- assets the whole video always uses
    pkid              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    calendar_id       UUID NOT NULL REFERENCES calendars(pkid),
    video_template_id UUID NOT NULL REFERENCES video_templates(pkid),
    asset_id          UUID NOT NULL REFERENCES assets(pkid),
    notes             TEXT,
    needed_before     INTERVAL NOT NULL DEFAULT INTERVAL '0' CHECK (needed_before >= INTERVAL '0'),
    archived_at       TIMESTAMPTZ,
    FOREIGN KEY (video_template_id, calendar_id) REFERENCES video_templates (pkid, calendar_id),
    FOREIGN KEY (asset_id, calendar_id)          REFERENCES assets (pkid, calendar_id)
);
CREATE UNIQUE INDEX ON video_template_assets (video_template_id, asset_id) WHERE archived_at IS NULL;

CREATE TABLE segment_template_assets (          -- e.g. the standard intro always uses the intro music
    pkid                UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    calendar_id         UUID NOT NULL REFERENCES calendars(pkid),
    segment_template_id UUID NOT NULL REFERENCES segment_templates(pkid),
    asset_id            UUID NOT NULL REFERENCES assets(pkid),
    notes               TEXT,
    needed_before       INTERVAL NOT NULL DEFAULT INTERVAL '0' CHECK (needed_before >= INTERVAL '0'),
    archived_at         TIMESTAMPTZ,
    FOREIGN KEY (segment_template_id, calendar_id) REFERENCES segment_templates (pkid, calendar_id),
    FOREIGN KEY (asset_id, calendar_id)            REFERENCES assets (pkid, calendar_id)
);
CREATE UNIQUE INDEX ON segment_template_assets (segment_template_id, asset_id) WHERE archived_at IS NULL;

-- Milestones are T-minus offsets, exactly as on real videos: 'Thumbnail ready, 2 days 4 hours
-- before finish'.
CREATE TABLE milestone_templates (
    pkid              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    video_template_id UUID NOT NULL REFERENCES video_templates(pkid),
    name              TEXT NOT NULL,
    before_finish     INTERVAL NOT NULL CHECK (before_finish >= INTERVAL '0'),
    archived_at       TIMESTAMPTZ
);
CREATE UNIQUE INDEX ON milestone_templates (video_template_id, lower(name)) WHERE archived_at IS NULL;

-- Publishing defaults: one row per channel the new video should go to, with its go-live offset,
-- visibility and text. A channel with no row starts as 'Skipped' on the new video.
CREATE TABLE publication_templates (
    pkid              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    calendar_id       UUID NOT NULL REFERENCES calendars(pkid),
    video_template_id UUID NOT NULL REFERENCES video_templates(pkid),
    channel_id        UUID NOT NULL REFERENCES channels(pkid),
    go_live_after     INTERVAL NOT NULL DEFAULT INTERVAL '0' CHECK (go_live_after >= INTERVAL '0'),
    visibility_id     UUID NOT NULL REFERENCES visibilities(pkid),
    is_premiere       BOOLEAN NOT NULL DEFAULT FALSE,
    title             TEXT,                  -- NULL = use the video's title
    description       TEXT,                  -- e.g. the standard links-and-credits footer
    tags              TEXT[] NOT NULL DEFAULT '{}',
    archived_at       TIMESTAMPTZ,
    FOREIGN KEY (video_template_id, calendar_id) REFERENCES video_templates (pkid, calendar_id),
    FOREIGN KEY (channel_id, calendar_id)        REFERENCES channels (pkid, calendar_id)
);
CREATE UNIQUE INDEX ON publication_templates (video_template_id, channel_id) WHERE archived_at IS NULL;

-- A series template starts a new series (a new show or season) with its description, cadence,
-- how many slots to create, and the video template its episodes start from.
CREATE TABLE series_templates (
    pkid              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    calendar_id       UUID NOT NULL REFERENCES calendars(pkid),
    name              TEXT NOT NULL,
    description       TEXT,
    video_template_id UUID NOT NULL REFERENCES video_templates(pkid),
    cadence           INTERVAL CHECK (cadence > INTERVAL '0'),
    default_slots     INT CHECK (default_slots BETWEEN 1 AND 500),  -- e.g. 10, 20 or 30 episodes
    archived_at       TIMESTAMPTZ,
    FOREIGN KEY (video_template_id, calendar_id) REFERENCES video_templates (pkid, calendar_id)
);
CREATE UNIQUE INDEX ON series_templates (calendar_id, lower(name)) WHERE archived_at IS NULL;

-- ===== No deletes, anywhere =====
-- A grant alone cannot stop a table owner, so this is one of the two places a trigger is used.
-- The app's database login has no DELETE right on any of these tables, and this trigger
-- refuses a delete even from a login that has one. Only a database owner deliberately
-- disabling the trigger (for example, a legal erasure request) can remove a row.
CREATE FUNCTION forbid_delete() RETURNS trigger LANGUAGE plpgsql AS $$
BEGIN
    RAISE EXCEPTION 'rows in % cannot be deleted; archive, end-date or deactivate them instead', TG_TABLE_NAME;
END $$;

DO $$
DECLARE t TEXT;
BEGIN
    FOREACH t IN ARRAY ARRAY['users', 'system_roles',
                             'groups', 'group_members', 'roles', 'role_permissions',
                             'workflow_states', 'workflow_state_usage', 'platforms',
                             'video_formats', 'segment_kinds', 'asset_kinds', 'visibilities', 'time_zones',
                             'calendars', 'channels', 'calendar_grants', 'channel_grants',
                             'series', 'videos', 'segments', 'assets', 'video_assets',
                             'segment_assets', 'publications', 'milestones',
                             'workflow_transitions', 'video_templates', 'segment_templates',
                             'video_template_segments', 'video_template_assets',
                             'segment_template_assets', 'milestone_templates',
                             'publication_templates', 'series_templates'] LOOP
        EXECUTE format('CREATE TRIGGER no_delete BEFORE DELETE ON %I
                        FOR EACH ROW EXECUTE FUNCTION forbid_delete()', t);
    END LOOP;
END $$;

-- ===== Workflow steps follow the table =====
-- A foreign key cannot compare a row's old and new state, so this is the third trigger.
-- A new row must start in its type's initial state; a change must be an active step in
-- workflow_transitions. The app shows only those steps as buttons, so normally this never fires.
CREATE FUNCTION check_workflow_step() RETURNS trigger LANGUAGE plpgsql AS $$
BEGIN
    IF TG_OP = 'INSERT' THEN
        IF NOT EXISTS (SELECT 1 FROM workflow_state_usage
                        WHERE object_type = NEW.state_scope AND state_id = NEW.state_id AND is_initial) THEN
            RAISE EXCEPTION 'a new % must start in its initial state', NEW.state_scope;
        END IF;
    ELSIF NEW.state_id IS DISTINCT FROM OLD.state_id THEN
        IF NOT EXISTS (SELECT 1 FROM workflow_transitions
                        WHERE object_type = NEW.state_scope AND from_state_id = OLD.state_id
                          AND to_state_id = NEW.state_id AND is_active) THEN
            RAISE EXCEPTION 'that % cannot move from its current state to the one asked for', NEW.state_scope;
        END IF;
    END IF;
    RETURN NEW;
END $$;

DO $$
DECLARE t TEXT;
BEGIN
    FOREACH t IN ARRAY ARRAY['videos', 'segments', 'assets', 'publications'] LOOP
        EXECUTE format('CREATE TRIGGER workflow_step BEFORE INSERT OR UPDATE OF state_id ON %I
                        FOR EACH ROW EXECUTE FUNCTION check_workflow_step()', t);
    END LOOP;
END $$;

-- ===== Calendar dates, worked out from each video's finish =====
-- The only stored plan date is videos.finish_at; everything else is computed here, so it
-- always agrees with it. Done and published moments are facts and win when present.
CREATE VIEW calendar_entries AS
SELECT v.calendar_id, v.pkid AS video_id, 'finish'::text AS kind, NULL::uuid AS item_id,
       v.working_title AS label, v.finish_at AS due_at, NULL::timestamptz AS done_at
  FROM videos v
 WHERE v.archived_at IS NULL
UNION ALL
SELECT v.calendar_id, v.pkid, 'milestone', m.pkid, m.name,
       v.finish_at - m.before_finish, m.done_at
  FROM milestones m JOIN videos v ON v.pkid = m.video_id
 WHERE m.archived_at IS NULL AND v.archived_at IS NULL
UNION ALL
SELECT p.calendar_id, p.video_id, 'go_live', p.pkid, ch.name,
       v.finish_at + p.go_live_after, p.published_at
  FROM publications p
  JOIN videos v    ON v.pkid = p.video_id
  JOIN channels ch ON ch.pkid = p.channel_id
  JOIN workflow_states ws ON ws.pkid = p.state_id
 WHERE p.archived_at IS NULL AND v.archived_at IS NULL AND ws.code NOT IN ('not_needed', 'removed')
UNION ALL
-- Stage deadlines: the current stage and every later one that has a T-minus.
SELECT v.calendar_id, v.pkid, 'stage', nxt.pkid, COALESCE(nxt.label, ws.label),
       v.finish_at - nxt.done_before_finish, NULL::timestamptz
  FROM videos v
  JOIN workflow_state_usage cur ON cur.state_id = v.state_id AND cur.object_type = 'video'
  JOIN workflow_state_usage nxt ON nxt.object_type = 'video' AND nxt.sort_order >= cur.sort_order
                               AND nxt.done_before_finish IS NOT NULL
  JOIN workflow_states ws ON ws.pkid = nxt.state_id
 WHERE v.archived_at IS NULL;

-- ===== Effective access, with the days each row is valid =====
-- valid = the overlap of the user's employment, the membership or role, and the grant.
-- channel_id NULL = the level applies to the whole calendar. Archived groups, roles,
-- calendars and channels grant nothing.
CREATE VIEW effective_access AS
SELECT sr.user_id, cal.pkid AS calendar_id, NULL::uuid AS channel_id, t.object_type,
       (CASE sr.role WHEN 'admin' THEN 'edit' ELSE 'read' END)::access_level AS level,
       tstzrange(sr.starts_at, sr.ends_at, '[)')
         * u.employment AS valid
  FROM system_roles sr
  JOIN users u              ON u.pkid = sr.user_id
  JOIN calendars cal        ON cal.archived_at IS NULL     -- admins and auditors: every calendar
  CROSS JOIN unnest(enum_range(NULL::object_type)) AS t(object_type)
UNION ALL
SELECT gm.user_id, g.calendar_id, NULL::uuid, rp.object_type, rp.level,
       tstzrange(gm.starts_at, gm.ends_at, '[)')
         * tstzrange(g.starts_at, g.ends_at, '[)')
         * u.employment
  FROM calendar_grants g
  JOIN groups gr            ON gr.pkid = g.group_id     AND gr.archived_at IS NULL
  JOIN roles r              ON r.pkid = g.role_id       AND r.archived_at IS NULL
  JOIN calendars cal        ON cal.pkid = g.calendar_id AND cal.archived_at IS NULL
  JOIN group_members gm     ON gm.group_id = g.group_id
  JOIN users u              ON u.pkid = gm.user_id
  JOIN role_permissions rp  ON rp.role_id = g.role_id
UNION ALL
SELECT gm.user_id, g.calendar_id, g.channel_id, rp.object_type, rp.level,
       tstzrange(gm.starts_at, gm.ends_at, '[)')
         * tstzrange(g.starts_at, g.ends_at, '[)')
         * u.employment
  FROM channel_grants g
  JOIN groups gr            ON gr.pkid = g.group_id     AND gr.archived_at IS NULL
  JOIN roles r              ON r.pkid = g.role_id       AND r.archived_at IS NULL
  JOIN calendars cal        ON cal.pkid = g.calendar_id AND cal.archived_at IS NULL
  JOIN channels ch          ON ch.pkid = g.channel_id   AND ch.archived_at IS NULL
  JOIN group_members gm     ON gm.group_id = g.group_id
  JOIN users u              ON u.pkid = gm.user_id
  JOIN role_permissions rp  ON rp.role_id = g.role_id;

-- Lookups: every foreign key column is indexed (Postgres does not do this by itself);
-- normal pages only read live rows, so most indexes skip archived ones. The lookup tables
-- are a few rows each, so their own foreign keys need no index.
CREATE INDEX ON system_roles      (user_id);
CREATE INDEX ON group_members     (user_id);
CREATE INDEX ON calendar_grants   (group_id);
CREATE INDEX ON calendar_grants   (calendar_id);
CREATE INDEX ON channel_grants    (group_id);
CREATE INDEX ON channel_grants    (channel_id);
CREATE INDEX ON videos            (calendar_id, state_id)     WHERE archived_at IS NULL;
CREATE INDEX ON videos            (calendar_id, finish_at)    WHERE archived_at IS NULL;
CREATE INDEX ON segments          (video_id)                  WHERE archived_at IS NULL;
CREATE INDEX ON assets            (calendar_id, state_id)     WHERE archived_at IS NULL;
CREATE INDEX ON video_assets      (asset_id);                 -- 'what uses this asset?'
CREATE INDEX ON segment_assets    (asset_id);
CREATE INDEX ON publications      (channel_id, state_id)      WHERE archived_at IS NULL;
CREATE INDEX ON publications      (video_id);
CREATE INDEX ON milestones        (video_id)                  WHERE archived_at IS NULL;
```

A typical read, listing the videos the logged-in user may see on the calendar:

```sql
SELECT v.pkid, v.working_title, v.state_id, v.finish_at
  FROM videos v
 WHERE v.calendar_id = $1
   AND v.archived_at IS NULL                     -- normal pages skip archived rows
   AND EXISTS (SELECT 1 FROM effective_access ea
                WHERE ea.user_id = $2            -- from AuthSession, never from the request
                  AND ea.valid @> now()          -- right now
                  AND ea.calendar_id = v.calendar_id
                  AND ea.channel_id IS NULL      -- video access comes from calendar-wide grants
                  AND ea.object_type = 'video'
                  AND ea.level >= 'read');
```

### Report: what uses this asset?

Because videos share assets, the asset page will have a **Where used** report (planned, not v1 unless you want it). It lists every video, and the segment if one is named, that uses the asset, with the video's stage and when the asset is needed for it. Archived uses are hidden unless the viewer ticks "include archived", which is how someone can check whether a logo or a licensed track is still in use before replacing it or letting the licence lapse. The index on `video_assets (asset_id)` makes this a quick lookup, and the same calendar access check as every other page applies.

```sql
SELECT v.pkid AS video_id, v.working_title, NULL AS segment, va.notes,
       v.finish_at - va.needed_before AS needed_by,
       (va.archived_at IS NOT NULL OR v.archived_at IS NOT NULL) AS archived
  FROM video_assets va
  JOIN videos v ON v.pkid = va.video_id
 WHERE va.asset_id = $1
   AND va.calendar_id = ANY($2)          -- calendars the user may read videos in, right now
   AND ($3 OR (va.archived_at IS NULL AND v.archived_at IS NULL))   -- $3 = include archived
UNION ALL
SELECT v.pkid, v.working_title, s.title, sa.notes,
       v.finish_at - sa.needed_before,
       (sa.archived_at IS NOT NULL OR s.archived_at IS NOT NULL OR v.archived_at IS NOT NULL)
  FROM segment_assets sa
  JOIN segments s ON s.pkid = sa.segment_id
  JOIN videos v   ON v.pkid = s.video_id
 WHERE sa.asset_id = $1
   AND sa.calendar_id = ANY($2)
   AND ($3 OR (sa.archived_at IS NULL AND s.archived_at IS NULL AND v.archived_at IS NULL))
 ORDER BY needed_by, working_title;
```

### Planning backwards from the finish date

Every video has one stored plan date, its **finish date** (`finish_at`, T-0): the moment it must be finished and ready to publish. Everything else on the calendar is counted from it.

- **Milestones count back.** A milestone stores how long before finish it is due, such as "Script locked, 10 days before". Its date on screen is the finish date minus that.
- **Assets count back from each use.** A video or segment that uses an asset says how long before finish it needs it. An asset shared by several videos is needed by the earliest of those moments, which the Where used report shows.
- **Go-live counts forward.** Each publication stores how long after finish it goes live on its channel, so YouTube can go out at finish and Facebook a day later.
- **Move the finish date and everything moves.** No other plan date is stored, so nothing can be left behind. The `calendar_entries` view works out the dates for the calendar.
- **What happened stays put.** A milestone's done time and a publication's published time are facts and never move.
- **Even a pitch has a finish date.** It means "if we make this, when would it need to be done", which also shows how soon a decision is needed.

**Stages have deadlines too.** Each video stage carries a T-minus, such as "Pre-production done 14 days before finish", so the calendar shows when every remaining step of a video is due and flags the ones running late. The deadlines live with the stages in `workflow_state_usage` (first guesses in the seed data), so changing them is an admin setting, not a code change.

The open edge is fixed appointments, such as a shoot booked with a guest, which should not move when the finish date does. See Q29.

### Series cadence and slots

A series can have a **cadence**, such as weekly, every two weeks or monthly. When you create a series, or later, you ask for a number of **slots**, say 10, 20 or 30, and the app creates that many videos straight away: "Episode 1", "Episode 2" and so on, each in the Pitch stage with its finish date one cadence after the last. A slot is not a new kind of thing. It is an ordinary video, so it has a finish date from the start, its stages and milestones count back from it, and it shows on the calendar at once.

After that the slots are just rows. Moving one episode's finish date moves only that episode and everything tied to it; the screen also offers "move this and every later episode" for when the whole run slips (guess). Changing the cadence never rewrites existing slots; it only sets the dates of slots added later. A series template can carry a cadence and a default number of slots, so a new season comes with its run already laid out.

```sql
-- Add $4 slots to series $1, using format $2, the first finishing at $3.
INSERT INTO videos (calendar_id, series_id, episode_number, working_title,
                    format_id, state_id, finish_at)
SELECT s.calendar_id, s.pkid, n, 'Episode ' || n, $2,
       (SELECT state_id FROM workflow_state_usage WHERE object_type = 'video' AND is_initial),
       $3::timestamptz + (n - 1) * s.cadence
  FROM series s, generate_series(1, $4::int) AS n
 WHERE s.pkid = $1 AND s.cadence IS NOT NULL;
```

New series are made with a wizard: pick a series template or start blank, name the series, choose the video template its episodes start from, set the cadence, the first finish date and the number of slots, then review the laid-out dates and confirm. On confirm, one transaction creates the series and every slot video, copying each slot's segments, asset links, milestones and publications from the chosen video template. That copy happens once, at creation; later edits to the template do not change the episodes, and more slots added later go through the same step.

### Templates

Work is often made to the same pattern, so a calendar keeps **templates** for the things made most often: videos, segments, series and publishing settings.

- **Video templates** hold format, brief and script, a rundown of library segments, the assets the video always uses, milestones timed before finish and publishing defaults.
- **Segment templates** sit on their own in the calendar's segment library ("Standard intro", "Sponsor read"). You can drop one into any video, and video templates build their rundown from them. Each carries its script, running time and the assets it always uses.
- **Publishing defaults** are part of a video template: one row per channel the video should go to, with how long after finish it goes live, its visibility, and standard title, description and tags. Channels left out start as Skipped.
- **Series templates** start a new show or season with its description and the video template its episodes start from.
- **Save as template.** From any video, the app copies its segments, asset links, milestones and publications into a new template. Dates are already offsets from the finish date, so nothing needs converting.
- **New video from template.** You give the new video a finish date, and the app copies everything in, with every date counted from it.
- **No link afterwards.** Editing a template never changes videos already made from it. The audit log records which template a video started from.
- **Can't be published by mistake.** Templates live in their own tables with no workflow state or publications, so the database cannot schedule one.
- **Access.** Templates belong to a calendar and use the `template` object type, so a role can let someone use templates without letting them edit them.

**Editor project files (a note only, not in the schema yet).** A segment, and a segment template, could later carry the editing project for that part of the video, such as a Kdenlive project (a .kdenlive XML file), in a large text column, so a new video starts with its timeline already set up. The segment tables are unchanged for now. See Q28.

### Can this user do this, right now?

Every request answers one question before it acts: does the signed-in user hold the needed level on this type of object, here, right now? If yes, the action runs and the triggers log it in the same transaction. If no, the app returns 403 and logs the refusal.

1. On each request, check the user's employment (`started_at` to `ended_at`) covers the current moment. If not, end the session.
2. Load the user's current access once per request: `SELECT calendar_id, channel_id, object_type, max(level) FROM effective_access WHERE user_id = $1 AND valid @> now() GROUP BY 1, 2, 3`. For a normal person that is tens of rows, found through the `user_id` indexes on `system_roles` and `group_members`.
3. Keep those rows in the request as a small map. Each `require_access(...)` check is then a lookup in memory, with no extra query.
4. List pages put the same rule in SQL (for example `calendar_id = ANY($readable_calendars)`), so rows a user cannot see are never loaded.
5. Nothing is cached between requests, so an ended role, membership or user stops working on the very next click.

Performance: my estimate is one extra indexed query of well under a millisecond per request at company scale (hundreds of users, dozens of calendars and groups). That is a guess to confirm with a load test. If it ever shows up in profiles, the next step is a per-user cache cleared whenever grants, memberships or roles change.

Composite foreign keys close the cross-parent gaps natively: a video or segment can only use an asset from its own calendar, a segment only belongs to a video on its calendar, a video only at a series on its calendar, a publication only at a channel on its video's calendar, and grants and group memberships only within one company.

## Accounts and sign-in

Only admins create users; there is no sign-up page. People sign in with a login name an admin assigns, not their email. Emails are only for notifications, so two users may share one.

### Account attributes

| Attribute | Column | Meaning |
| --- | --- | --- |
| Login name | `login_name` | Assigned by an admin. Unique among users whose employment periods overlap. |
| Display name | `display_name` | The name shown in the app and the audit log. |
| Email | `email` | Optional, for notifications only. Duplicates allowed. |
| Password | `password_hash` | Argon2id one-way hash in PHC format, which embeds a random salt per user. Empty until a password is set. |
| Login account | `can_login` | Off for people tracked in the app who never sign in, such as an outside voice-over artist. They can have no password. |
| Must change password | `must_change_password` | Forces a new password at next sign-in. On for every new user and after every admin reset. |
| Locked | `locked_at`, `locked_by`, `locked_reason` | Set by an admin. Blocks sign-in until an admin clears it. |
| Temporary lockout | `failed_logins`, `locked_until` | After 5 wrong passwords in a row, sign-in is refused for 15 minutes (guess, Q25). |
| Password changed | `password_changed_at` | When the password was last set. Passwords do not expire; they change only when the person chooses or an admin resets them. |
| Last sign-in | `last_login_at` | A convenience copy; the audit log is the real record. |
| Two-factor | `totp_secret_enc` | Authenticator-app secret, encrypted with a key kept outside the database (Q17). This is the only secret the server must be able to decrypt, because it needs it to check codes; it is not a password. |
| Employment | `started_at`, `ended_at` | One stretch with the company. For someone known to be leaving, an admin sets ended\_at ahead of time (for example 5:00 PM on their last day) and the account stops working by itself at that moment; nobody has to remember to lock it. |
| Time zone | `time_zone_id` | How every time is shown to this person. The admin who creates the user picks a starting zone (the form defaults to the instance's home zone), and the user can change it any time on their own settings page. Only real zones from the time\_zones list can be stored. |

### How sign-in decides

Sign-in succeeds only when all of these hold: the login name matches a user whose period covers the current moment, `can_login` is on, the account is not locked or in a lockout, and the password verifies against the hash. Every failure shows the same message ("Login name or password is wrong") and is logged with its real reason. A success resets `failed_logins`, issues a new session id and, if `must_change_password` is on, allows nothing but the change-password page.

### Passwords

- Hashed with Argon2id through the `password-auth` crate the repo already uses; each hash carries its own random salt. Nothing reversible is ever stored, and passwords never appear in logs or the audit log.
- Decent passwords are enforced on every set or change, following NIST SP 800-63B: at least 12 characters and at most 128; any characters including spaces, so passphrases work; no forced mix of symbols and digits; refused if it appears in a list of known-breached passwords, contains the login name or display name, or scores below 3 of 4 on the zxcvbn strength check (guess on the thresholds). The form says which rule failed. Passwords never expire.
- An admin reset generates a random temporary password, shows it to the admin once, stores only its hash, turns on `must_change_password` and ends the user's open sessions.
- Self-service reset by email is left out for now, since emails are not unique (Q24).

### The first admin

The migration that creates the tables also creates one admin user, `admin`, with no password, so nobody can sign in yet. Running `media-calendar-manager bootstrap-admin` once on the server then sets a random one-time password, prints it to that terminal only, and forces a change at first sign-in. The command refuses to run once the admin has a password, and it writes an audit event.

```sql
-- In the migration: the first admin, who cannot sign in until bootstrap-admin runs
WITH u AS (INSERT INTO users (login_name, display_name, can_login, must_change_password, time_zone_id)
           SELECT 'admin', 'Administrator', TRUE, TRUE, pkid FROM time_zones WHERE code = 'UTC'
           RETURNING pkid, started_at, employment)
INSERT INTO system_roles (user_id, role, starts_at, employment)
SELECT pkid, 'admin', started_at, employment FROM u;
```

The existing register page and route would be removed. This assumes the current `users` table holds only test accounts.

To keep the company from locking itself out, the app refuses any change that would leave no current admin who can sign in: an end date, a lock, or the end of an admin assignment. Every other user can be end-dated or locked freely.

## Audit log

Every change to content or access, and every sign-in event, writes one row to `audit_events`, which nobody can edit or delete. Changes are captured by database triggers, so a handler that forgets to log cannot leave a gap; sign-in events are written by the app because they happen outside any table change.

### What gets logged

| Event | Written by | Details kept |
| --- | --- | --- |
| Create, update, delete on any content table | Trigger | Changed fields, old and new values |
| Changes to users, groups, roles, grants | Trigger | Who gained or lost what access |
| Login success and failure | App | Login name tried, IP, user agent; never the password |
| Logout and session expiry | App | Session reference (hashed) |
| Password change or reset, lock and unlock, user created or end-dated | App | Who did it and to whom |
| Access denied (a user tried something their role forbids) | App | Object and level asked for |

### How it works

- Each request runs in a transaction that first sets `app.user_id`, `app.ip` and `app.request_id` with `set_config(..., true)`. The triggers read these, so every row names the person who made the change.
- The app's database login can only `INSERT` and `SELECT` on `audit_events`; a trigger also rejects `UPDATE` and `DELETE`. Migrations run under a separate owner login.
- Each row stores a hash of the previous row (`prev_hash`), so any later tampering breaks the chain and shows up in a check (guess: optional, cheap to add now).
- Admins and auditors read the log through an Activity page filtered by date, person or object. A calendar-scoped auditor sees only that calendar's content changes, not sign-ins.
- Rows are kept for a set period, then archived (Q16).

```sql
CREATE TABLE audit_events (
    pkid          BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    occurred_at   TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    calendar_id   UUID REFERENCES calendars(pkid),   -- set for content changes; scopes calendar auditors
    actor_user_id UUID REFERENCES users(pkid),       -- NULL for a failed login on an unknown email
    action        TEXT NOT NULL,                   -- 'video.update', 'auth.login_failed', 'grant.create'
    object_type   TEXT,                            -- table name
    object_id     UUID,
    changes       JSONB,                           -- {field: [old, new]} for updates
    ip_address    INET,
    user_agent    TEXT,
    request_id    UUID,
    prev_hash     BYTEA,
    row_hash      BYTEA NOT NULL
);
CREATE INDEX ON audit_events (occurred_at DESC);
CREATE INDEX ON audit_events (calendar_id, occurred_at DESC);
CREATE INDEX ON audit_events (object_type, object_id);
CREATE INDEX ON audit_events (actor_user_id, occurred_at DESC);
```

## Security baseline

A company tool holding unreleased content and client work needs these from the first release. Items marked (guess) are my defaults; the rest are standard practice.

| Area | Practice | Status today |
| --- | --- | --- |
| Sign-up | No sign-up: only admins create users and assign login names | Public register page exists; would change |
| Passwords | Argon2id one-way hashes with a per-user salt, never reversible; at least 12 characters, checked against known-breached passwords (guess) | Argon2 via `password-auth` already |
| Two-factor | TOTP codes (authenticator app), required for admins, optional for others (guess, Q17) | Not built |
| Brute force | Rate-limit login by IP and by account; slow down after 5 failures; same error text for a wrong login name and a wrong password | Not built |
| Sessions | Secure, HttpOnly, SameSite=Lax cookies; new session id on login; 8-hour idle and 7-day absolute timeout (guess); log out all sessions on password change | Cookies via `tower-sessions`; timeouts to set |
| CSRF | A per-session token in every form and HTMX request, checked on every POST | Not built |
| Transport | HTTPS only, HSTS header | TLS via rustls already |
| Browser headers | Content-Security-Policy limited to self plus the pinned Pico/HTMX CDN files with integrity hashes; `frame-ancestors 'none'`; `Referrer-Policy: same-origin` | Not built |
| Output | Askama auto-escaping left on; no raw HTML from users | Default already |
| Access checks | Every query goes through `effective_access`; denied attempts logged | This design |
| Leavers | Users are never deleted; leaving sets an end date, after which their sessions and access stop | This design |
| Database | App login has no DDL rights and cannot alter the audit log; backups encrypted | To set up |
| Dependencies | `cargo audit` and `cargo deny` in CI | Not set up |

## Proposed CLAUDE.md changes

Rule 4 in the repo's CLAUDE.md says every query must filter by `user_id`. Shared ownership breaks that rule, so it needs replacing before any of this is built. Proposed wording:

```markdown
4. **Multiuser Security:** Content belongs to the company, not a user. Every query for
   content MUST (a) check the `AuthSession` user's employment covers now(), and (b) require the needed level from
   `effective_access` for that object type on its calendar or channel: `read` to view,
   `edit` to create, change or delete. Never take a user, calendar or channel
   id from the request as proof of access. Admin-only actions (calendars, channels, groups,
   roles, grants) need a current admin assignment. Reading the audit log needs admin,
   auditor, or `read` on `audit_log` for that calendar. Auditors never get `edit`.
   "Current" always means valid at now(). Store every date and time as TIMESTAMPTZ
   (UTC), never DATE, and show it in the viewer's own time zone.
   **No deletes:** Never write `DELETE` for app data. Hide things by setting `archived_at`
   (who did it is in the audit log); end people and their roles with an end date. Normal pages filter
   `archived_at IS NULL`; only the audit view shows archived rows.
6. **Audit:** Every write runs in a transaction that first sets `app.user_id`, `app.ip` and
   `app.request_id`, so the audit triggers can record who did it. Auth events (login,
   failed login, logout, password change) and access-denied responses are logged by the
   app. Never update or delete `audit_events`.
7. **Security:** Every POST carries and checks a CSRF token. Never disable Askama escaping.
   New CDN scripts or styles need a pinned version and an integrity hash. Passwords,
   temporary ones included, are only stored as Argon2id hashes and never logged.
   There is no sign-up route; only admins create users.
8. **No impossible states:** The database must not be able to hold data that makes no
   sense. Enforce every rule in the schema first: structure (required links, separate
   tables instead of optional parents, composite foreign keys), then CHECK, UNIQUE and
   EXCLUDE constraints. Use a trigger only when none of those can express it. Rust code may
   add friendlier checks in front, but never replaces a database rule.
```

I would put the access check in one helper (for example `require_access(&auth, ObjectType::Video, AccessLevel::Edit, channel_id)` returning `AppError::Forbidden`), so handlers cannot each re-invent it.

## Notes for the user guide

These rules come from the data model and will surprise people unless the help pages and the screens explain them. Each line is written the way it should read in the user guide.

- **Every video lives in a series.** For a one-off video, put it in your calendar's "One-offs" series (or any catch-all series you create). Each new calendar starts with a "One-offs" series so there is always somewhere to put it. (guess: the app creates it automatically)
- **Nothing is deleted, only archived.** Archiving hides an item from normal pages. Auditors can still see it, and an admin can bring it back.
- **Archiving a video doesn't archive what's inside it.** Its segments, publications and milestones stay as they were, and they come back when the video is un-archived. The assets it used stay in the asset library for other videos.
- **Assets are shared.** An asset belongs to the calendar, not to one video. Before changing or replacing one, open its **Where used** report to see which videos use it.
- **Access dates sit inside employment.** A person's roles and group memberships can't start before their start date or run past their end date. To end someone's employment earlier, end their roles first. The screen will offer to do this for you.
- **People who return get a new account.** A returning employee gets a new user, and may get their old login name back. Their earlier history stays with their old user.
- **Times are shown in your own time zone.** Your admin sets a starting zone when creating your account; change it any time under your settings. Every date has a time too, so a due date can be 8:00 AM on the day something must be ready, and it appears in your own zone.

* **A pitch is just a video at its first stage.** Write the brief, give it a rough finish date, and green-light it when it is approved; it keeps all its history. A turned-down pitch moves to Dropped.
* **Dates follow the finish date.** Milestones, asset needs and go-live times are set as "so long before (or after) finish". Change a video's finish date and all of them move with it. Done and published times never move.
* **Only allowed steps are offered.** The buttons on an item show the steps your company's workflow allows from where it is now. An admin can change the workflow.

- **Slots are real episodes.** Creating a series with 20 slots creates 20 videos with dates from the cadence. Move any of them when life happens; moving one doesn't move the others unless you choose "this and every later episode".
- **Every stage has a due time.** Each stage of a video is due a set time before its finish date, so the calendar can warn you early when a video is falling behind.

## Open questions

Your answers decide the first migration. Where I had to pick, the draft uses the choice in brackets.

- [ ] **Q1. One video, several channels?** Answered: yes. A video goes to every channel on its calendar by default, and skipping some is normal. Draft: \[Yes: one `publications` row per channel.\]
- [ ] **Q2. Reusable assets?** Do intro music, logos or lower-third templates get reused across many videos, and should the app track them once? Answered: yes. Assets belong to the calendar's asset library and videos share them through video\_assets. The asset usage report shows where each one is used.
- [ ] **Q3. Just you, or a team?** Answered: it is a company tool with groups, roles and per-type access. Still open: should work be assigned to named people, and should approvals be recorded as sign-offs? \[Not yet; the audit log shows who changed the stage.\]
- [ ] **Q4. Scenes and shots?** Do you plan at the level of individual shots (a shot list), or are segments enough? \[Segments are enough.\]
- [ ] **Q5. How much rights tracking?** Is a source, terms and an expiry date on each asset enough, or do you need licences as their own records with documents? \[Three columns on `assets`.\]
- [ ] **Q6. Where do files live?** Should the app hold uploaded files, or only link to Drive, Dropbox or a local path? \[Links only.\]
- [ ] **Q7. Sync with YouTube and Facebook?** Should the app later read or push schedules through the platforms' APIs, or is manual entry fine? \[Manual entry; `platform_url` is stored for later.\]
- [ ] **Q8. Dependencies between milestones?** Do you need "B-roll shoot must finish before picture lock" rules, or just dated checkpoints? \[Dated checkpoints only.\]
- [ ] **Q9. Do the seven stages match how you work?** States now live in a lookup table, so they can be renamed, added or switched off later without a code change; the shared-state map above is a first guess at the names. The allowed steps between them are a first guess too, and also live in a table.

Added after your notes on shared access, auditing and the calendar:

- [ ] **Q10. Roles, or levels set per grant?** The draft uses reusable roles (Editor, Viewer, Thumbnail designer). The alternative is to set the seven levels directly on each grant: simpler tables, more clicking.
- [ ] **Q11. Can a grant take access away?** \[No: the highest level from any group wins, and there is no deny.\]
- [ ] **Q12. Cross-posting and access.** Should someone granted only the Facebook channel see the videos themselves (script, assets), or just the Facebook publications? \[Just the publications; video access comes from a calendar-wide grant.\]
- [ ] **Q13. Grants below the channel?** Do you ever need to share a single series or video with someone, such as a client reviewing one episode? \[No: calendar and channel only.\]
- [ ] **Q14. One company per installation?** Answered: yes. Each company gets its own installation and database, so a breach or bug cannot spill from one company into another.
- [ ] **Q15. Who manages access?** \[Only admins create calendars, channels, groups, roles and grants.\]
- [ ] **Q16. How long is the audit log kept?** \[Forever in the database for now; archiving after a set period can be added.\]
- [ ] **Q17. Two-factor sign-in.** \[Required for admins, optional for everyone else.\] Should it be required for all?
- [ ] **Q18. What is a calendar to you?** One per client, per brand, or per team? Can one channel appear on two calendars? \[An installation has several calendars, and each channel belongs to exactly one.\]

* [ ] **Q19. How wide is the audit role?** \[Two options, both in the draft: an Auditor who reads everything and the whole log, and a role with read on `audit_log` for one calendar.\] Is the instance-wide one enough? Should an admin also be barred from being an auditor?

- [ ] **Q20. Log reads too?** Should viewing a page be logged, or only changes, sign-ins and refused attempts? \[Only changes, sign-ins and refusals; logging every view multiplies the log many times over.\]
- [ ] **Q21. When does a date take effect?** \[Answered: every date has a time and is stored in UTC, shown in each user's own zone. A due date is a moment, such as 8:00 AM on a given day; an end date is the moment access stops.\]
- [ ] **Q22. Returning employees.** Their new user starts clean. Should the app show a link to their earlier user, for example on the audit view? \[No link in v1.\]
- [ ] **Q23. Archiving a parent.** Answered: archiving never cascades. An archived video's segments, publications and milestones stay as they are, unreachable from normal pages, and the assets it used stay in the library for other videos. Un-archiving the video brings everything back as it was.

* [ ] **Q24. Forgotten passwords.** \[An admin resets them.\] Should people be able to reset their own by email later, given that emails can be shared?
* [ ] **Q25. Lockout rules.** \[5 wrong passwords in a row lock sign-in for 15 minutes; admins can unlock early.\] Different numbers, or a lock that only an admin can clear?

- [ ] **Q26. Password expiry.** Answered: no expiry. Strong passwords are enforced instead (see Passwords).

* [ ] **Q27. What else needs templates?** Answered: videos, segments, series and publishing defaults. All four are in the draft.

- [ ] **Q28. Editor project files on segments.** Should a segment, and a segment template, hold its Kdenlive project (or another editor's) so a new video opens with its timeline ready? It could be a large text column, or a link to a file kept next to the footage. \[Noted only; segment tables unchanged.\]
- [ ] **Q29. Fixed appointments.** Some dates should not move when the finish date does, such as a shoot booked with a guest. Should those be a separate kind of milestone pinned to a real date, with a warning when the finish date moves past it? \[Not yet: every milestone counts back from finish.\]
- [ ] **Q30. One workflow, or several?** The steps now live in a table shared by the whole instance. Do some calendars or series need their own workflow, such as Shorts skipping Review? \[One workflow per type for the whole instance.\]
- [ ] **Q31. Should a series remember its template?** Answered: the new-series wizard creates the episodes from a template video at creation time, and nothing links them afterwards.

* [ ] **Q32. Stage deadlines per format?** A Short and a long-form video need very different timelines, but the draft has one set of stage deadlines for the whole instance. Should video templates (or formats) carry their own? \[One set for now; per-template deadlines are the next step.\]
* [ ] **Q33. Cadences that one interval can't say.** "Every Tuesday and Thursday" or "first Monday of the month" don't fit a single interval. Do you need those? \[A single interval; for two a week, make two series or move slots by hand.\]
