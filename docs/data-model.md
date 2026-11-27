# Media Calendar: Glossary and Data Model (v1)

Nov 27, 2026 · @Frederick Price

## Summary

The app runs one company's video production. Each company gets its own installation and database, so nothing can leak between companies. The company has one or more **content calendars**; each calendar holds **channels** and series; every video belongs to a **series**, breaks into **segments**, and tracks its **assets** until it is **published** on one or more platforms. People sit in **groups**, and each group gets a **role** on a whole calendar or on one channel, which sets edit, read or none for each type of object from there down. Every change and sign-in is written to an **audit log**. Each video is planned backwards from its finish date, so moving that date moves everything tied to it. Times are stored in UTC and shown in each person's own time zone. That is 39 content tables (11 of them for templates), 7 access tables, 10 small lookup and workflow tables and 1 audit table beside the existing `users` table.

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
| Cadence | When a series releases: a cycle (a week, two weeks, a month) with one or more release times in it, such as Tuesdays and Thursdays at 10:00. Used to lay out slots. | `series_cadences, cadence_times` |
| Slot | A planned episode created ahead of time from the cadence, holding its place on the calendar until its idea is filled in. A slot is an ordinary video in the Pitch stage, with a finish date that can be moved. | `videos` |
| Video | One finished piece of content. An *episode* is a video in a series. Every video belongs to a series; loose one-off videos go in a catch-all series such as "One-offs". | `videos` |
| Format | The shape of a video: long-form, Short/Reel, or live stream. | `videos.format` |
| Pitch / brief | A short statement of the idea, audience and angle, written before any work starts. A pitch is not a separate kind of thing: it is a video in its first state, Pitch, and green-lighting it moves the same video on. | `videos.brief` |
| Milestone | A checkpoint on the way to finishing, set as a time before the video's finish date (T-minus), such as script done 10 days before, shoot day, picture lock. It moves when the finish date moves. | `milestones` |
| Appointment | Something booked at a fixed time, such as a shoot with a guest or a studio slot. Unlike a milestone it stays put when the finish date moves, but it has a T-minus: how long before the finish it must be over. | `appointments` |
| Resource | A studio or room that can only be in one time slot at a time. Appointments book resources, and a resource can never be booked into overlapping times. Locations don't clash. | `resources, appointment_resources` |
| Crew | A named team of users, such as Crew A. Booking a crew books each person on it, so people see their own appointments (a crew lead sees the whole crew's), and nobody can be in two overlapping time slots even if they are on two crews. | `crews, crew_members, appointment_people` |
| Equipment model / item | A model is a kind of gear (Sony FX3). An item is one piece of it with its own asset tag number, printed on a label, and is tracked on its own so wear, damage and where it has been are known. Each item moves through a workflow: Ready, Out on a job, Needs cleaning, Needs restocking, Damaged, Out for repair, Missing. Every model has a kind (crate, case or bag, gear, accessory, consumable): pieces sit inside or attach to others (a battery on a camera, the camera in its bag, the bag in a crate), and consumables such as AA batteries are counted, not tagged. | `equipment_kinds, equipment_models, equipment_items, equipment_contents, equipment_issues` |
| Package template | A kit list for a kind of job, such as Interview kit: models and how many, never serial numbers. Video templates can bring kits along. The same gear can be packed by many templates. | `package_templates, package_template_items` |
| Package | One crate packed for one appointment: what it needs (copied from a template) and the tagged items and counted consumables actually in it. Ticked off on screen as it is packed, and prints as a checklist. An item can't be in two packages at overlapping times. | `packages, package_needs, package_items`, package\_supplies |
| Kit | A real, persistent crate (Interview kit A) that keeps its gear between jobs and names the kit list it should hold. The crate is itself a tagged item, and the kit is whatever is inside it now. Booking a kit packs the crate and everything in it for an appointment. | `kits, kit_bookings` |
| Finish date (T-0) | The moment a video must be finished and ready to publish. Every video has one, even a pitch. Milestones and asset needs count back from it (T-minus) and go-live times count forward (T-plus), so moving it moves the whole plan. | `videos.finish_at` |
| Template | A reusable starting point. A video template holds format, brief, script, a rundown of library segments, the assets it always uses, milestones timed before finish and publishing defaults per channel. Segment templates (a standard intro, a sponsor read) and series templates work the same way. Saved from an existing item or built by hand; a template can never be published itself. | `video_templates` and its child tables |
| Workflow | A named, versioned set of stages, deadlines and allowed steps for one type of item, such as Standard or Short. Picked from a list when an item is created. A published version never changes; a change is a new version. | `workflows` |

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
| Admin | A user who creates calendars, channels, groups and roles and hands out access. Admins can edit everything. An auditor can read everything, including the full audit log, and change nothing. A workflow editor builds and publishes workflows, and gets no other access from that role. | `system_roles` |
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

Videos, segments, assets and publications all follow the same plan, make, check, release pattern, so they share one `workflow_states` lookup table. A second small table, `workflow_state_usage`, says which states each workflow version uses, in what order, what to call them there and by when. Each row stores the state's `pkid`; a composite key makes sure, for example, that a segment can only hold a state listed for segments. Every state has an `is_active` flag: switching one off hides it from new choices without breaking rows that already use it.

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

The workflow is data, and there can be several. A **workflow** is a named, versioned set of stages and allowed steps for one type of item, such as "Standard" or "Short" for videos. You pick one from a list when you create an item; the type's default is offered first, and a video template names the workflow its videos use. Each version lists its stages (`workflow_state_usage`, with each stage's deadline) and its allowed steps (`workflow_transitions`, with the button text people see, such as "Green-light", "Send back to edit" or "Skip").

- **Draft, then published.** An admin builds or edits a version as a draft. Publishing it freezes it for good, so a video always follows exactly the rules it was planned under. To change a workflow, copy it into the next version, edit the draft and publish that.
- **Items stay on their version.** Videos already under way keep the version they started on. An admin can move them to a newer version, one at a time or in bulk, as long as their current stage exists there; the stage never jumps during a move.
- **Old versions stay.** An old version can be archived so nobody picks it for new work, but it is never deleted, and the audit log shows which version every item followed.
- **The database holds the line.** The screen shows only the steps allowed from where an item is now, and triggers refuse anything else: a new item must start in the first stage of a published workflow, each change must be a step listed in its version, and a published version cannot be edited.

A pitch that is turned down moves to **Dropped**, so it stays on record and out of the way.

## Proposed Postgres schema

Fifty-nine tables: forty for content (eleven of them templates) with `calendars` at the top and `videos` at the centre, seven for access, eleven small lookup and workflow tables, and one append-only audit log. No table names a company: each company has its own installation and database, and its name and home time zone are settings. The calendar is drawn from one stored plan date, `videos.finish_at`: milestones, asset needs and go-live times are offsets from it, worked out by the `calendar_entries` view.

**Design rule: the database must not be able to hold data that makes no sense.** Wherever possible the structure itself rules a bad state out: required links instead of optional ones, separate tables instead of a column that means "one of two things", composite foreign keys so related rows must share a calendar or company, and CHECK, UNIQUE and exclusion constraints for everything else. A trigger is used only when a rule truly cannot be written that way; today that is the audit log, the no-delete guard, the check that a state change follows its workflow, the freeze that stops a published workflow version from changing, and the check that only Ready gear is packed. App code and screens may get more complex to keep admins' lives easy, but they never replace a database rule. They only add friendly checks in front of it.

&#91;embedded content: tables and links · 7 tables\]

A publication joins a video to a channel, so one video can go out on YouTube and Facebook with its own title and date on each.

### Ownership, access and safety rules

- **Nothing with history is deleted.** Every table blocks `DELETE` twice: the app's database login lacks the right, and a trigger refuses it anyway. Only a database owner who deliberately switches the trigger off can remove a row. Because creating a row is itself logged, every row has audit history from its first moment, so "no deletes at all" is the same rule as "no deletes once audited", and simpler to build.
- **Archive instead.** People-facing tables have `one archived_at column; who archived a row, like who created or changed it, is only in the audit log`. Archiving is one click for anyone with `edit` on that object type, is logged, and can be undone. Archived rows vanish from normal pages; the audit view still shows them. Archiving never cascades: an archived video's segments and milestones stay as they were, simply unreachable from normal pages, and the assets it used stay in the calendar's asset library for other videos.
- **A user is one stretch of employment.** Users have `started_at` and an optional `ended_at`, both full timestamps. Someone who leaves and comes back gets a new user; the same login name may be reused because the two periods cannot overlap.
- **Roles are dated too.** Admin and auditor assignments, group memberships and grants each have `starts_at` and an optional `ends_at`. Dates can be edited, every edit is logged, and exclusion constraints refuse any change that would make two periods for the same thing overlap. No assignment or membership can start before the user's start date or end after their end date, and the database itself refuses it, so a coding mistake cannot open that hole. Each dated row carries a copy of the user's employment range; a composite foreign key with ON UPDATE CASCADE keeps the copy equal to the user's real dates, and a CHECK keeps the row's period inside it. Shortening a user's end date is refused while any of their roles still runs later, so the app ends those roles first in the same transaction. As a second guard, access is still intersected with the employment dates.
- **Native constraints, not triggers.** Every foreign key references its parent's `pkid`. Where two parents must agree, such as a publication's video and channel being on the same calendar, the child also carries the shared column and a composite foreign key enforces it against a `UNIQUE (pkid, calendar_id)` on the parent. Triggers are kept only for the audit log, the no-delete guard, the workflow-step check and the workflow-version freeze, which constraints cannot express.
- The access tree is: calendar, then series, then videos (every video is in a series), then segments, assets and milestones. Channels also sit under the calendar, and a publication joins a video to one of them. Each channel sits in exactly one calendar (guess, Q18).
- A video publishes to every channel on its calendar by default: creating it adds a draft publication for each channel. Leaving a channel out is normal and is recorded by setting that publication to `skipped`, with an optional reason.
- A grant gives a group a role either on a whole calendar (`calendar_grants`) or on one channel (`channel_grants`). They are separate tables, so no grant can be unclear about what it covers, and who manages a channel can change without touching the calendar's grants. A channel-only grant covers that channel and its publications (guess, Q12).
- `effective_access` lists every level a user holds and the period it is valid (`valid`). The question asked is always: does this user hold the needed level here, right now?
- When a person holds several levels, the highest wins; there is no explicit deny (guess, Q11).
- Admins have `edit` on everything and manage calendars, channels, groups, roles and grants. Auditors have `read` on everything, including the full audit log, and change nothing. A role with `read` on `audit_log` makes a calendar-only auditor (Q19). Workflow editors build and publish the company's one list of workflows; that role gives no access to content.
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
--    rule; today that is the audit log, the no-delete guard, the workflow-step check, the
--    freeze on published workflow versions and the ready-gear check when packing.
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
                                  'asset', 'publication', 'milestone', 'template', 'audit_log',
                                  'equipment');
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

-- Workflows are versioned and chosen from a list. Each row is one version of one named workflow
-- for one type ('Long-form' v3 for videos). A draft (published_at NULL) can be edited; once
-- published it is frozen, and a change means a new version. Items keep the version they use.
CREATE TABLE workflows (
    pkid         UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    object_type  object_type NOT NULL
                 CHECK (object_type IN ('video', 'segment', 'asset', 'publication', 'equipment')),
    name         TEXT NOT NULL,             -- 'Standard', 'Short', 'Live stream'
    version      INT  NOT NULL CHECK (version > 0),
    description  TEXT,
    published_at TIMESTAMPTZ,               -- NULL = draft; set = frozen and offered in the list
    is_default   BOOLEAN NOT NULL DEFAULT FALSE,   -- offered first for its type
    created_at   TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    archived_at  TIMESTAMPTZ,               -- no longer offered; items already on it keep it
    UNIQUE (object_type, name, version),
    UNIQUE (pkid, object_type),             -- lets a video insist on a video workflow
    CHECK (NOT is_default OR published_at IS NOT NULL)
);
CREATE UNIQUE INDEX ON workflows (object_type) WHERE is_default AND archived_at IS NULL;

-- The states one workflow version uses, in what order, under what name, and by when.
CREATE TABLE workflow_state_usage (
    pkid        UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    workflow_id UUID NOT NULL REFERENCES workflows(pkid),
    state_id    UUID NOT NULL REFERENCES workflow_states(pkid),
    label       TEXT,                     -- NULL = the state's own label; e.g. 'Skipped' for publications
    sort_order  INT  NOT NULL,
    is_initial  BOOLEAN NOT NULL DEFAULT FALSE,   -- where a new item starts
    done_before_finish INTERVAL CHECK (done_before_finish >= INTERVAL '0'),
                        -- T-minus: leave this state this long before the finish; NULL = no deadline
    UNIQUE (workflow_id, state_id),       -- target for the items' and steps' foreign keys
    UNIQUE (workflow_id, sort_order)
);
CREATE UNIQUE INDEX ON workflow_state_usage (workflow_id) WHERE is_initial;  -- one start each

-- Each row is one allowed step in one workflow version. Both ends must be states that version
-- uses (composite keys); a step back, such as Review to Post-production, is just another row.
-- The trigger near the end refuses any other change.
CREATE TABLE workflow_transitions (
    pkid          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    workflow_id   UUID NOT NULL REFERENCES workflows(pkid),
    from_state_id UUID NOT NULL,
    to_state_id   UUID NOT NULL,
    label         TEXT,                     -- button text, e.g. 'Send back to edit'
    sort_order    INT  NOT NULL,
    CHECK (from_state_id <> to_state_id),
    UNIQUE (workflow_id, from_state_id, to_state_id),
    FOREIGN KEY (workflow_id, from_state_id) REFERENCES workflow_state_usage (workflow_id, state_id),
    FOREIGN KEY (workflow_id, to_state_id)   REFERENCES workflow_state_usage (workflow_id, state_id)
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
    ('removed', 'Removed', 130),
    ('in_use', 'Out on a job', 140),      ('needs_cleaning', 'Needs cleaning', 150),
    ('needs_restock', 'Needs restocking', 160), ('damaged', 'Damaged', 170),
    ('in_repair', 'Out for repair', 180), ('missing', 'Missing', 190);

-- Version 1 of a 'Standard' workflow for each type, and a 'Short' one for videos with tighter
-- deadlines. Created as drafts, filled in, then published.
INSERT INTO workflows (object_type, name, version) VALUES
    ('video', 'Standard', 1), ('video', 'Short', 1), ('segment', 'Standard', 1),
    ('asset', 'Standard', 1), ('publication', 'Standard', 1), ('equipment', 'Standard', 1);

INSERT INTO workflow_state_usage (workflow_id, state_id, label, sort_order, is_initial)
SELECT w.pkid, s.pkid, u.label, u.sort_order, u.is_initial
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
    ('removed',         'publication', NULL,        50, FALSE),
    ('ready',           'equipment',   NULL,        10, TRUE),
    ('in_use',          'equipment',   NULL,        20, FALSE),
    ('needs_cleaning',  'equipment',   NULL,        30, FALSE),
    ('needs_restock',   'equipment',   NULL,        40, FALSE),
    ('damaged',         'equipment',   NULL,        50, FALSE),
    ('in_repair',       'equipment',   NULL,        60, FALSE),
    ('missing',         'equipment',   NULL,        70, FALSE)
  ) AS u(code, object_type, label, sort_order, is_initial)
  JOIN workflow_states s ON s.code = u.code
  JOIN workflows w ON w.object_type = u.object_type::object_type AND w.name = 'Standard';

-- First guess at the allowed steps; admins change them by publishing a new version.
INSERT INTO workflow_transitions (workflow_id, from_state_id, to_state_id, label, sort_order)
SELECT w.pkid, f.pkid, d.pkid, t.label, t.sort_order
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
    ('publication', 'not_needed', 'planned', 'Un-skip', 60),
    ('equipment', 'ready', 'in_use', 'Check out', 10),
    ('equipment', 'in_use', 'ready', 'Back, all good', 20),
    ('equipment', 'in_use', 'needs_cleaning', 'Back, needs cleaning', 30),
    ('equipment', 'in_use', 'needs_restock', 'Back, needs restocking', 40),
    ('equipment', 'needs_cleaning', 'needs_restock', 'Cleaned', 50),
    ('equipment', 'needs_cleaning', 'ready', 'Cleaned, ready', 60),
    ('equipment', 'needs_restock', 'ready', 'Restocked', 70),
    ('equipment', 'ready', 'damaged', 'Report damage', 80),
    ('equipment', 'in_use', 'damaged', 'Report damage', 90),
    ('equipment', 'needs_cleaning', 'damaged', 'Report damage', 100),
    ('equipment', 'needs_restock', 'damaged', 'Report damage', 110),
    ('equipment', 'damaged', 'in_repair', 'Send for repair', 120),
    ('equipment', 'damaged', 'ready', 'Fine after all', 130),
    ('equipment', 'in_repair', 'ready', 'Back from repair', 140),
    ('equipment', 'in_use', 'missing', 'Not returned', 150),
    ('equipment', 'missing', 'ready', 'Found', 160),
    ('equipment', 'missing', 'damaged', 'Found damaged', 170)
  ) AS t(object_type, from_code, to_code, label, sort_order)
  JOIN workflow_states f ON f.code = t.from_code
  JOIN workflow_states d ON d.code = t.to_code
  JOIN workflows w ON w.object_type = t.object_type::object_type AND w.name = 'Standard';

-- 'Short' starts as a copy of the standard video workflow.
INSERT INTO workflow_state_usage (workflow_id, state_id, label, sort_order, is_initial)
SELECT sh.pkid, u.state_id, u.label, u.sort_order, u.is_initial
  FROM workflow_state_usage u
  JOIN workflows st ON st.pkid = u.workflow_id AND st.object_type = 'video' AND st.name = 'Standard'
  JOIN workflows sh ON sh.object_type = 'video' AND sh.name = 'Short';
INSERT INTO workflow_transitions (workflow_id, from_state_id, to_state_id, label, sort_order)
SELECT sh.pkid, t.from_state_id, t.to_state_id, t.label, t.sort_order
  FROM workflow_transitions t
  JOIN workflows st ON st.pkid = t.workflow_id AND st.object_type = 'video' AND st.name = 'Standard'
  JOIN workflows sh ON sh.object_type = 'video' AND sh.name = 'Short';

-- First guess at each video stage's deadline, counted back from the finish (all guesses).
UPDATE workflow_state_usage u SET done_before_finish = d.before::interval
  FROM (VALUES ('Standard', 'pitch', '28 days'), ('Standard', 'pre_production', '14 days'),
               ('Standard', 'production', '7 days'), ('Standard', 'post_production', '3 days'),
               ('Standard', 'in_review', '1 day'), ('Standard', 'ready', '0'),
               ('Short', 'pitch', '7 days'), ('Short', 'pre_production', '4 days'),
               ('Short', 'production', '3 days'), ('Short', 'post_production', '1 day'),
               ('Short', 'in_review', '12 hours'), ('Short', 'ready', '0')
       ) AS d(workflow, code, before)
  JOIN workflow_states s ON s.code = d.code
  JOIN workflows w ON w.object_type = 'video' AND w.name = d.workflow
 WHERE u.state_id = s.pkid AND u.workflow_id = w.pkid;

UPDATE workflows SET published_at = NOW(), is_default = (name = 'Standard');

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
CREATE TABLE system_roles (                   -- dated company-wide roles: admin, auditor, workflow editor
    pkid       UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id    UUID NOT NULL REFERENCES users(pkid),
    role       TEXT NOT NULL CHECK (role IN ('admin', 'auditor', 'workflow_editor')),
                                            -- workflow_editor: may build and publish workflows
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
-- Rows with a workflow state carry workflow_id, a fixed workflow_type and state_id. One composite
-- key means a video can only follow a video workflow; another, only hold a state its workflow uses.
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
    created_at   TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    archived_at  TIMESTAMPTZ,
    UNIQUE (pkid, calendar_id)
);
CREATE UNIQUE INDEX ON series (calendar_id, lower(title)) WHERE archived_at IS NULL;   -- no twin series

-- A series' rhythm: a cycle (a week, two weeks, a month) and one or more release times inside it,
-- so 'Tuesdays and Thursdays at 10:00' is a 7-day cycle starting Monday 00:00 with times at
-- '1 day 10:00' and '3 days 10:00'. A series with no row here has no rhythm (e.g. 'One-offs').
-- Used only when the app lays out slots; each slot is then an ordinary video row of its own.
CREATE TABLE series_cadences (
    pkid         UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    series_id    UUID NOT NULL UNIQUE REFERENCES series(pkid),   -- at most one per series
    cycle        INTERVAL NOT NULL CHECK (cycle > INTERVAL '0'), -- '7 days', '14 days', '1 month'
    starts_at    TIMESTAMPTZ NOT NULL,     -- start of the first cycle, e.g. a Monday 00:00
    time_zone_id UUID NOT NULL REFERENCES time_zones(pkid),  -- cycles follow this zone's clock, so
                                           -- 10:00 stays 10:00 across daylight saving
    UNIQUE (pkid, cycle)
);

CREATE TABLE cadence_times (
    pkid            UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    cadence_id      UUID NOT NULL REFERENCES series_cadences(pkid),
    cycle           INTERVAL NOT NULL,     -- copy of the cadence's cycle, kept equal by the key below
    offset_in_cycle INTERVAL NOT NULL CHECK (offset_in_cycle >= INTERVAL '0'),  -- '1 day 10:00'
    archived_at     TIMESTAMPTZ,
    FOREIGN KEY (cadence_id, cycle) REFERENCES series_cadences (pkid, cycle) ON UPDATE CASCADE,
    CHECK (offset_in_cycle < cycle)        -- every release time falls inside its cycle
);
CREATE UNIQUE INDEX ON cadence_times (cadence_id, offset_in_cycle) WHERE archived_at IS NULL;

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
    workflow_id       UUID NOT NULL REFERENCES workflows(pkid),  -- picked from the list
    workflow_type     object_type NOT NULL DEFAULT 'video' CHECK (workflow_type = 'video'),
    state_id          UUID NOT NULL REFERENCES workflow_states(pkid),
    brief             TEXT,                -- the pitch; a pitch is a video in the 'Pitch' state
    script            TEXT,
    finish_at         TIMESTAMPTZ NOT NULL,  -- T-0: when it must be finished, ready to publish.
                                         -- For a pitch, when it would need to be finished.
    created_at        TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at        TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    archived_at       TIMESTAMPTZ,
    UNIQUE (pkid, calendar_id),
    FOREIGN KEY (series_id, calendar_id) REFERENCES series (pkid, calendar_id),  -- series on same calendar
    FOREIGN KEY (workflow_id, workflow_type) REFERENCES workflows (pkid, object_type),
    FOREIGN KEY (workflow_id, state_id)      REFERENCES workflow_state_usage (workflow_id, state_id)
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
    workflow_id     UUID NOT NULL REFERENCES workflows(pkid),  -- picked from the list
    workflow_type   object_type NOT NULL DEFAULT 'segment' CHECK (workflow_type = 'segment'),
    state_id        UUID NOT NULL REFERENCES workflow_states(pkid),
    archived_at     TIMESTAMPTZ,
    UNIQUE (pkid, calendar_id),
    FOREIGN KEY (video_id, calendar_id) REFERENCES videos (pkid, calendar_id),
    FOREIGN KEY (workflow_id, workflow_type) REFERENCES workflows (pkid, object_type),
    FOREIGN KEY (workflow_id, state_id)      REFERENCES workflow_state_usage (workflow_id, state_id),
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
    workflow_id        UUID NOT NULL REFERENCES workflows(pkid),  -- picked from the list
    workflow_type      object_type NOT NULL DEFAULT 'asset' CHECK (workflow_type = 'asset'),
    state_id           UUID NOT NULL REFERENCES workflow_states(pkid),
    location           TEXT,               -- URL or file path; the app stores no files (Q6)
                                           -- no due date here: each use says when it is needed
    notes              TEXT,
    license_source     TEXT,               -- rights, guess (Q5)
    license_terms      TEXT,
    license_expires_at TIMESTAMPTZ,
    created_at         TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    archived_at        TIMESTAMPTZ,
    UNIQUE (pkid, calendar_id),
    FOREIGN KEY (workflow_id, workflow_type) REFERENCES workflows (pkid, object_type),
    FOREIGN KEY (workflow_id, state_id)      REFERENCES workflow_state_usage (workflow_id, state_id)
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
    workflow_id   UUID NOT NULL REFERENCES workflows(pkid),  -- picked from the list
    workflow_type object_type NOT NULL DEFAULT 'publication' CHECK (workflow_type = 'publication'),
    state_id      UUID NOT NULL REFERENCES workflow_states(pkid),
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
    FOREIGN KEY (workflow_id, workflow_type) REFERENCES workflows (pkid, object_type),
    FOREIGN KEY (workflow_id, state_id)      REFERENCES workflow_state_usage (workflow_id, state_id)
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

-- Appointments are pinned to real times (a shoot booked with a guest, a studio slot) and do not
-- move when the finish date does. A separate table from milestones, so neither has a column that
-- only sometimes applies. Each still has a T-minus: it must be over by finish_at - needed_before
-- (a shoot needed 10 days before finish, so editing has time). schedule_problems flags any that
-- end later than that, for example after the finish date moves earlier.
CREATE TABLE appointments (
    pkid        UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    video_id    UUID NOT NULL REFERENCES videos(pkid),
    name        TEXT NOT NULL,             -- 'Shoot with guest', 'Studio B'
    starts_at   TIMESTAMPTZ NOT NULL,
    ends_at     TIMESTAMPTZ NOT NULL,
    location    TEXT,                      -- free text; a location can host several shoots at once
    needed_before INTERVAL NOT NULL DEFAULT INTERVAL '0' CHECK (needed_before >= INTERVAL '0'),
                                           -- T-minus: must end by videos.finish_at - needed_before
    done_at     TIMESTAMPTZ,
    archived_at TIMESTAMPTZ,
    period      TSTZRANGE GENERATED ALWAYS AS (tstzrange(starts_at, ends_at, '[)')) STORED,
    is_live     BOOLEAN   GENERATED ALWAYS AS (archived_at IS NULL) STORED,
    CHECK (ends_at > starts_at),
    UNIQUE (pkid, period, is_live)         -- copied onto each booking below
);

-- Rooms and studios that can only be in one time slot at a time. People are booked below as
-- users, and gear as serial-numbered equipment packed into packages. A location is not a
-- resource: it can host several shoots at once. Company-wide, because a studio can serve
-- several calendars.
CREATE TABLE resources (
    pkid        UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name        TEXT NOT NULL,             -- 'Studio 2', 'Edit suite'
    notes       TEXT,
    archived_at TIMESTAMPTZ
);
CREATE UNIQUE INDEX ON resources (lower(name)) WHERE archived_at IS NULL;

-- Which resources an appointment books. Each booking carries a copy of its appointment's time
-- and live flag (kept equal by the composite key, ON UPDATE CASCADE), so one exclusion constraint
-- refuses any resource booked twice at overlapping times: moving or reviving an appointment into
-- a clash is refused too. Archiving the appointment or the booking frees the resource.
CREATE TABLE appointment_resources (
    pkid             UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    appointment_id   UUID NOT NULL REFERENCES appointments(pkid),
    resource_id      UUID NOT NULL REFERENCES resources(pkid),
    period           TSTZRANGE NOT NULL,
    appointment_live BOOLEAN NOT NULL,
    archived_at      TIMESTAMPTZ,
    FOREIGN KEY (appointment_id, period, appointment_live)
        REFERENCES appointments (pkid, period, is_live) ON UPDATE CASCADE,
    EXCLUDE USING gist (resource_id WITH =, period WITH &&)
        WHERE (appointment_live AND archived_at IS NULL)     -- one time slot at a time
);
CREATE UNIQUE INDEX ON appointment_resources (appointment_id, resource_id) WHERE archived_at IS NULL;

-- A crew is a named team of users ('Crew A'), company-wide. Membership is dated and sits inside
-- each person's employment, like group membership. A crew is a shortcut for booking: booking
-- 'Crew A' books each of its members at that moment, so a person in two crews still can't be
-- in two overlapping time slots.
CREATE TABLE crews (
    pkid        UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name        TEXT NOT NULL,
    archived_at TIMESTAMPTZ
);
CREATE UNIQUE INDEX ON crews (lower(name)) WHERE archived_at IS NULL;

CREATE TABLE crew_members (                    -- dated: who was on which crew, when
    pkid         UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    crew_id      UUID NOT NULL REFERENCES crews(pkid),
    user_id      UUID NOT NULL REFERENCES users(pkid),
    starts_at    TIMESTAMPTZ NOT NULL,
    ends_at      TIMESTAMPTZ,
    is_lead      BOOLEAN NOT NULL DEFAULT FALSE,  -- a lead sees every booking of the crew's members
    employment   TSTZRANGE NOT NULL,        -- copy of the person's employment
    CHECK (ends_at IS NULL OR ends_at > starts_at),
    FOREIGN KEY (user_id, employment) REFERENCES users (pkid, employment) ON UPDATE CASCADE,
    CHECK (employment @> tstzrange(starts_at, ends_at, '[)')),  -- inside employment
    EXCLUDE USING gist (crew_id WITH =, user_id WITH =,
                        tstzrange(starts_at, ends_at, '[)') WITH &&)
);

-- Which people an appointment books. Like appointment_resources, each row copies the
-- appointment's time and live flag, so one exclusion constraint refuses a person in two
-- overlapping time slots, whichever crews they came through. It also copies the person's
-- employment, so nobody can be booked outside their employment, and moving the appointment or
-- shortening the employment past a booking is refused. People see their own bookings from here.
CREATE TABLE appointment_people (
    pkid             UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    appointment_id   UUID NOT NULL REFERENCES appointments(pkid),
    user_id          UUID NOT NULL REFERENCES users(pkid),
    period           TSTZRANGE NOT NULL,
    appointment_live BOOLEAN NOT NULL,
    employment       TSTZRANGE NOT NULL,    -- copy of the person's employment
    archived_at      TIMESTAMPTZ,
    FOREIGN KEY (appointment_id, period, appointment_live)
        REFERENCES appointments (pkid, period, is_live) ON UPDATE CASCADE,
    FOREIGN KEY (user_id, employment) REFERENCES users (pkid, employment) ON UPDATE CASCADE,
    CHECK (employment @> period),          -- booked only while employed
    EXCLUDE USING gist (user_id WITH =, period WITH &&)
        WHERE (appointment_live AND archived_at IS NULL)     -- one time slot at a time
);
CREATE UNIQUE INDEX ON appointment_people (appointment_id, user_id) WHERE archived_at IS NULL;

-- ===== Equipment =====
-- Gear is tracked piece by piece, because there may be several of the same thing and each one
-- wears out or breaks on its own. A model is the kind of thing ('Sony FX3', 'Rode NTG5'); an item
-- is one piece of it. Each item gets an asset tag number from the database, printed on a label
-- and stuck on the gear, so nobody has to hunt for the maker's serial. Company-wide, like crews.
-- Every model has a kind, and the kind's rank says what can hold what: a lower rank holds a
-- higher one (a crate holds a case, a case holds a camera, a camera holds its battery). Ranks
-- leave gaps so a new kind ('Pouch', 25) fits between. Consumables carry no asset tag.
CREATE TABLE equipment_kinds (
    pkid       UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    code       TEXT NOT NULL UNIQUE,
    label      TEXT NOT NULL,
    sort_order INT  NOT NULL,
    is_active  BOOLEAN NOT NULL DEFAULT TRUE,
    nest_rank  SMALLINT NOT NULL,
    is_tagged  BOOLEAN  NOT NULL,           -- false = counted, never tracked piece by piece
    UNIQUE (pkid, nest_rank, is_tagged)     -- copied onto each model below
);
INSERT INTO equipment_kinds (code, label, sort_order, nest_rank, is_tagged) VALUES
    ('crate', 'Crate', 10, 10, TRUE),            ('case', 'Case or bag', 20, 20, TRUE),
    ('gear', 'Gear', 30, 30, TRUE),              ('accessory', 'Accessory', 40, 40, TRUE),
    ('consumable', 'Consumable', 50, 90, FALSE);  -- batteries, tape, gels

CREATE TABLE equipment_models (
    pkid        UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name        TEXT NOT NULL,
    kind_id     UUID NOT NULL,
    nest_rank   SMALLINT NOT NULL,          -- copy of the kind's
    is_tagged   BOOLEAN  NOT NULL,          -- copy of the kind's
    notes       TEXT,
    archived_at TIMESTAMPTZ,
    FOREIGN KEY (kind_id, nest_rank, is_tagged)
        REFERENCES equipment_kinds (pkid, nest_rank, is_tagged) ON UPDATE CASCADE,
    UNIQUE (pkid, nest_rank, is_tagged),
    UNIQUE (pkid, is_tagged)
);
CREATE UNIQUE INDEX ON equipment_models (lower(name)) WHERE archived_at IS NULL;

CREATE TABLE equipment_items (
    pkid          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    model_id      UUID NOT NULL REFERENCES equipment_models(pkid),
    nest_rank     SMALLINT NOT NULL,        -- copy of the model's
    is_tagged     BOOLEAN NOT NULL DEFAULT TRUE CHECK (is_tagged),  -- consumables have no items
    asset_tag     BIGINT NOT NULL GENERATED ALWAYS AS IDENTITY UNIQUE,  -- on the label; never reused
    serial_number TEXT,                     -- the maker's serial, if known
    nickname      TEXT,                     -- 'Camera B'
    workflow_id   UUID NOT NULL REFERENCES workflows(pkid),
    workflow_type object_type NOT NULL DEFAULT 'equipment' CHECK (workflow_type = 'equipment'),
    state_id      UUID NOT NULL REFERENCES workflow_states(pkid),  -- Ready, Out on a job,
                                            -- Needs cleaning, Needs restocking, Damaged, Out for repair
    acquired_at   TIMESTAMPTZ,
    archived_at   TIMESTAMPTZ,              -- retired, sold or written off
    FOREIGN KEY (workflow_id, workflow_type) REFERENCES workflows (pkid, object_type),
    FOREIGN KEY (workflow_id, state_id)      REFERENCES workflow_state_usage (workflow_id, state_id),
    FOREIGN KEY (model_id, nest_rank, is_tagged)
        REFERENCES equipment_models (pkid, nest_rank, is_tagged) ON UPDATE CASCADE,
    UNIQUE (pkid, nest_rank)                -- copied onto equipment_contents below
);
CREATE UNIQUE INDEX ON equipment_items (model_id, lower(serial_number));  -- even once retired

-- What is inside or attached to what, and since when: a battery on a camera, the camera in its
-- case, the case in a crate. A piece is in one place at a time. Each row carries both ranks and
-- the holder must rank lower, so ranks only go up along a chain and nothing can end up inside
-- itself, however deep. Changing a model's kind is refused if it would break that.
CREATE TABLE equipment_contents (
    pkid           UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    item_id        UUID NOT NULL,
    item_rank      SMALLINT NOT NULL,
    container_id   UUID NOT NULL,
    container_rank SMALLINT NOT NULL,
    added_at       TIMESTAMPTZ NOT NULL DEFAULT now(),
    removed_at     TIMESTAMPTZ,
    FOREIGN KEY (item_id, item_rank) REFERENCES equipment_items (pkid, nest_rank) ON UPDATE CASCADE,
    FOREIGN KEY (container_id, container_rank)
        REFERENCES equipment_items (pkid, nest_rank) ON UPDATE CASCADE,
    CHECK (container_rank < item_rank),     -- so no loops
    CHECK (removed_at IS NULL OR removed_at > added_at),
    EXCLUDE USING gist (item_id WITH =, tstzrange(added_at, removed_at, '[)') WITH &&)  -- one place at a time
);

-- Wear and damage reports. Where an item has been is its packing history (package_items);
-- this records what was found. An open issue shows on the problems report while the item is
-- packed for an appointment still to come.
CREATE TABLE equipment_issues (
    pkid          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    item_id       UUID NOT NULL REFERENCES equipment_items(pkid),
    reported_at   TIMESTAMPTZ NOT NULL DEFAULT now(),
    description   TEXT NOT NULL,            -- 'cracked hot shoe', 'battery holds 40%'
    resolved_at   TIMESTAMPTZ,
    archived_at   TIMESTAMPTZ,
    CHECK (resolved_at IS NULL OR resolved_at >= reported_at)
);

-- A package template is a kit list for a kind of job ('Interview kit', 'Run-and-gun'): models
-- and how many, never serial numbers. The same gear can be packed by many templates.
CREATE TABLE package_templates (
    pkid        UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name        TEXT NOT NULL,
    notes       TEXT,
    archived_at TIMESTAMPTZ
);
CREATE UNIQUE INDEX ON package_templates (lower(name)) WHERE archived_at IS NULL;

CREATE TABLE package_template_items (
    pkid                UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    package_template_id UUID NOT NULL REFERENCES package_templates(pkid),
    model_id            UUID NOT NULL REFERENCES equipment_models(pkid),
    quantity            INT  NOT NULL CHECK (quantity > 0),
    archived_at         TIMESTAMPTZ
);
CREATE UNIQUE INDEX ON package_template_items (package_template_id, model_id) WHERE archived_at IS NULL;

-- A package is one crate packed for one appointment. 'Pack from template' copies the template's
-- list into package_needs (editable afterwards; nothing links back), then real items are packed
-- into package_items. Like a booking, a package carries a copy of its appointment's time and
-- live flag; its own is_live also goes false when the package is archived, and that flows on to
-- its items, so archiving a package or its appointment unpacks the gear.
CREATE TABLE packages (
    pkid             UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    appointment_id   UUID NOT NULL REFERENCES appointments(pkid),
    name             TEXT NOT NULL,         -- 'Crate 1: camera'
    period           TSTZRANGE NOT NULL,
    appointment_live BOOLEAN NOT NULL,
    archived_at      TIMESTAMPTZ,
    is_live          BOOLEAN GENERATED ALWAYS AS (appointment_live AND archived_at IS NULL) STORED,
    FOREIGN KEY (appointment_id, period, appointment_live)
        REFERENCES appointments (pkid, period, is_live) ON UPDATE CASCADE,
    UNIQUE (pkid, period, is_live)          -- copied onto each packed item below
);
CREATE UNIQUE INDEX ON packages (appointment_id, lower(name)) WHERE archived_at IS NULL;

CREATE TABLE package_needs (                    -- what the crate should hold, by model
    pkid        UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    package_id  UUID NOT NULL REFERENCES packages(pkid),
    model_id    UUID NOT NULL REFERENCES equipment_models(pkid),
    quantity    INT  NOT NULL CHECK (quantity > 0),
    archived_at TIMESTAMPTZ
);
CREATE UNIQUE INDEX ON package_needs (package_id, model_id) WHERE archived_at IS NULL;

-- The serial-numbered pieces actually in the crate. One exclusion constraint refuses a piece in
-- two packages at overlapping times, whichever appointments they are for; moving an appointment
-- or reviving a package into a clash is refused too. Packing a case packs everything in it as
-- rows of their own, so each piece is still booked, checked and missed one by one; scanning the
-- case out or in covers its contents in one go.
CREATE TABLE package_items (
    pkid         UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    package_id   UUID NOT NULL REFERENCES packages(pkid),
    item_id      UUID NOT NULL REFERENCES equipment_items(pkid),
    period       TSTZRANGE NOT NULL,
    package_live BOOLEAN NOT NULL,
    packed_at    TIMESTAMPTZ,               -- ticked on the checklist as it goes in the crate
    checked_out_at TIMESTAMPTZ,             -- scanned out when the crate leaves for the shoot
    checked_in_at  TIMESTAMPTZ,             -- scanned back in after it; who did each is in the
                                            -- audit log. Not back = 'Missing' and on the report
    archived_at  TIMESTAMPTZ,
    CHECK (checked_out_at IS NULL OR packed_at IS NOT NULL),      -- packed before it leaves
    CHECK (checked_in_at IS NULL OR checked_in_at >= checked_out_at), -- in only after out
    CHECK (checked_in_at IS NULL OR checked_out_at IS NOT NULL),
    FOREIGN KEY (package_id, period, package_live)
        REFERENCES packages (pkid, period, is_live) ON UPDATE CASCADE,
    EXCLUDE USING gist (item_id WITH =, period WITH &&)
        WHERE (package_live AND archived_at IS NULL)         -- one time slot at a time
);
CREATE UNIQUE INDEX ON package_items (package_id, item_id) WHERE archived_at IS NULL;

-- Consumables are counted lines on the checklist ('AA battery: 4'), ticked as they go in. They
-- are used up, so they are not checked back in; the crate goes to Needs restocking instead.
CREATE TABLE package_supplies (
    pkid        UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    package_id  UUID NOT NULL REFERENCES packages(pkid),
    model_id    UUID NOT NULL,
    is_tagged   BOOLEAN NOT NULL DEFAULT FALSE CHECK (NOT is_tagged),  -- consumables only
    quantity    INT  NOT NULL CHECK (quantity > 0),                    -- how many went in
    packed_at   TIMESTAMPTZ NOT NULL DEFAULT now(),
    archived_at TIMESTAMPTZ,
    FOREIGN KEY (model_id, is_tagged) REFERENCES equipment_models (pkid, is_tagged) ON UPDATE CASCADE
);
CREATE UNIQUE INDEX ON package_supplies (package_id, model_id) WHERE archived_at IS NULL;

-- The kits a video will need ('Interview kit'), usually copied from its video template. When
-- the video gets an appointment, its packages are packed from these.
CREATE TABLE video_kits (
    pkid                UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    video_id            UUID NOT NULL REFERENCES videos(pkid),
    package_template_id UUID NOT NULL REFERENCES package_templates(pkid),
    archived_at         TIMESTAMPTZ
);
CREATE UNIQUE INDEX ON video_kits (video_id, package_template_id) WHERE archived_at IS NULL;

-- A kit is a real, persistent crate ('Interview kit A') that keeps its gear between jobs. It
-- names the kit list it should hold. The crate is itself a tagged item, and what lives in it is
-- whatever equipment_contents puts inside it (and inside its cases) right now.
CREATE TABLE kits (
    pkid                UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name                TEXT NOT NULL,
    container_id        UUID NOT NULL REFERENCES equipment_items(pkid),  -- the crate
    package_template_id UUID NOT NULL REFERENCES package_templates(pkid),  -- what it should hold
    notes               TEXT,
    archived_at         TIMESTAMPTZ
);
CREATE UNIQUE INDEX ON kits (lower(name)) WHERE archived_at IS NULL;
CREATE UNIQUE INDEX ON kits (container_id) WHERE archived_at IS NULL;   -- one kit per crate

-- Booking a kit makes a package for the appointment from it: its kit list becomes the package's
-- needs and the crate and everything in it now are packed (all must be Ready, see the trigger).
-- This row says which kit a package came from, so the kit's booking history can be shown.
CREATE TABLE kit_bookings (
    pkid       UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    package_id UUID NOT NULL UNIQUE REFERENCES packages(pkid),   -- a package is one kit at most
    kit_id     UUID NOT NULL REFERENCES kits(pkid)
);

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
    workflow_id  UUID NOT NULL REFERENCES workflows(pkid),   -- e.g. 'Short' for a Shorts template
    workflow_type object_type NOT NULL DEFAULT 'video' CHECK (workflow_type = 'video'),
    brief        TEXT,
    script       TEXT,
    created_at   TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    archived_at  TIMESTAMPTZ,
    UNIQUE (pkid, calendar_id),
    FOREIGN KEY (workflow_id, workflow_type) REFERENCES workflows (pkid, object_type)
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

-- Kits a video template brings along, copied into video_kits for each new video.
CREATE TABLE video_template_kits (
    pkid                UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    video_template_id   UUID NOT NULL REFERENCES video_templates(pkid),
    package_template_id UUID NOT NULL REFERENCES package_templates(pkid),
    archived_at         TIMESTAMPTZ
);
CREATE UNIQUE INDEX ON video_template_kits (video_template_id, package_template_id)
    WHERE archived_at IS NULL;

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

-- A series template starts a new series (a new show or season) with its description, how many
-- slots to create, and the video template its episodes start from. The wizard asks for the cadence.
CREATE TABLE series_templates (
    pkid              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    calendar_id       UUID NOT NULL REFERENCES calendars(pkid),
    name              TEXT NOT NULL,
    description       TEXT,
    video_template_id UUID NOT NULL REFERENCES video_templates(pkid),
    default_slots     INT CHECK (default_slots BETWEEN 1 AND 500),  -- e.g. 10, 20 or 30 episodes
    archived_at       TIMESTAMPTZ,
    FOREIGN KEY (video_template_id, calendar_id) REFERENCES video_templates (pkid, calendar_id)
);
CREATE UNIQUE INDEX ON series_templates (calendar_id, lower(name)) WHERE archived_at IS NULL;


```

The schema continues: the no-delete guard, workflow triggers, views and indexes.

```sql
-- ===== No deletes, anywhere =====
-- A grant alone cannot stop a table owner, so this is one of the few places a trigger is used.
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
                             'series_cadences', 'cadence_times', 'appointments', 'resources',
                             'appointment_resources', 'crews', 'crew_members', 'appointment_people',
                             'equipment_kinds', 'equipment_models', 'equipment_items', 'equipment_issues',
                             'equipment_contents', 'package_supplies',
                             'package_templates', 'package_template_items', 'packages',
                             'package_needs', 'package_items', 'video_kits', 'video_template_kits',
                             'kits', 'kit_bookings',
                             'workflows', 'workflow_transitions', 'video_templates', 'segment_templates',
                             'video_template_segments', 'video_template_assets',
                             'segment_template_assets', 'milestone_templates',
                             'publication_templates', 'series_templates'] LOOP
        EXECUTE format('CREATE TRIGGER no_delete BEFORE DELETE ON %I
                        FOR EACH ROW EXECUTE FUNCTION forbid_delete()', t);
    END LOOP;
END $$;

-- ===== Workflow steps follow the table =====
-- A foreign key cannot compare a row's old and new values, so this is the third trigger.
-- A new item must use a published, current workflow and start in its first state; a state change
-- must be a step listed in that workflow version; moving an item to another workflow or version
-- keeps its state (the composite key already makes sure that state exists there).
CREATE FUNCTION check_workflow_step() RETURNS trigger LANGUAGE plpgsql AS $$
BEGIN
    IF TG_OP = 'INSERT' THEN
        IF NOT EXISTS (SELECT 1 FROM workflows w
                         JOIN workflow_state_usage u ON u.workflow_id = w.pkid
                        WHERE w.pkid = NEW.workflow_id AND w.published_at IS NOT NULL
                          AND w.archived_at IS NULL AND u.state_id = NEW.state_id AND u.is_initial) THEN
            RAISE EXCEPTION 'a new row in % must use a published workflow and start in its first state',
                TG_TABLE_NAME;
        END IF;
    ELSIF NEW.workflow_id IS DISTINCT FROM OLD.workflow_id THEN
        IF NEW.state_id IS DISTINCT FROM OLD.state_id
           OR NOT EXISTS (SELECT 1 FROM workflows WHERE pkid = NEW.workflow_id
                             AND published_at IS NOT NULL AND archived_at IS NULL) THEN
            RAISE EXCEPTION 'a row in % can only move to a published workflow, keeping its state',
                TG_TABLE_NAME;
        END IF;
    ELSIF NEW.state_id IS DISTINCT FROM OLD.state_id THEN
        IF NOT EXISTS (SELECT 1 FROM workflow_transitions
                        WHERE workflow_id = NEW.workflow_id AND from_state_id = OLD.state_id
                          AND to_state_id = NEW.state_id) THEN
            RAISE EXCEPTION 'that row in % cannot move from its current state to the one asked for',
                TG_TABLE_NAME;
        END IF;
    END IF;
    RETURN NEW;
END $$;

DO $$
DECLARE t TEXT;
BEGIN
    FOREACH t IN ARRAY ARRAY['videos', 'segments', 'assets', 'publications', 'equipment_items'] LOOP
        EXECUTE format('CREATE TRIGGER workflow_step BEFORE INSERT OR UPDATE OF workflow_id, state_id ON %I
                        FOR EACH ROW EXECUTE FUNCTION check_workflow_step()', t);
    END LOOP;
END $$;

-- ===== Published workflow versions are frozen =====
-- The fourth trigger: no constraint can say 'this row may change only while its parent is a draft'.
-- A published version keeps its states, steps and deadlines for good; only is_default, description
-- and archived_at may still change. To change a workflow, copy it into version n + 1.
CREATE FUNCTION freeze_published_workflow() RETURNS trigger LANGUAGE plpgsql AS $$
BEGIN
    IF TG_TABLE_NAME = 'workflows' THEN
        IF OLD.published_at IS NOT NULL
           AND (NEW.object_type, NEW.name, NEW.version, NEW.published_at)
               IS DISTINCT FROM (OLD.object_type, OLD.name, OLD.version, OLD.published_at) THEN
            RAISE EXCEPTION 'workflow % version % is published; make a new version instead',
                OLD.name, OLD.version;
        END IF;
    ELSIF EXISTS (SELECT 1 FROM workflows WHERE published_at IS NOT NULL
                     AND (pkid = NEW.workflow_id OR (TG_OP = 'UPDATE' AND pkid = OLD.workflow_id))) THEN
        RAISE EXCEPTION 'that workflow version is published; make a new version instead';
    END IF;
    RETURN NEW;
END $$;

CREATE TRIGGER frozen BEFORE UPDATE ON workflows
    FOR EACH ROW EXECUTE FUNCTION freeze_published_workflow();
CREATE TRIGGER frozen BEFORE INSERT OR UPDATE ON workflow_state_usage
    FOR EACH ROW EXECUTE FUNCTION freeze_published_workflow();
CREATE TRIGGER frozen BEFORE INSERT OR UPDATE ON workflow_transitions
    FOR EACH ROW EXECUTE FUNCTION freeze_published_workflow();

-- ===== Only Ready gear can be packed =====
-- The fifth trigger: whether an item may be packed depends on its state at that moment, which
-- no key can check. Packing (or un-archiving a packed row) needs the item in 'ready'. Gear that
-- breaks after it was packed stays packed and shows on the problems report instead.
CREATE FUNCTION check_gear_ready() RETURNS trigger LANGUAGE plpgsql AS $$
BEGIN
    IF NEW.archived_at IS NULL AND (TG_OP = 'INSERT' OR OLD.archived_at IS NOT NULL)
       AND NOT EXISTS (SELECT 1 FROM equipment_items i JOIN workflow_states ws ON ws.pkid = i.state_id
                        WHERE i.pkid = NEW.item_id AND ws.code = 'ready' AND i.archived_at IS NULL) THEN
        RAISE EXCEPTION 'only gear in the Ready state can be packed';
    END IF;
    RETURN NEW;
END $$;

CREATE TRIGGER gear_ready BEFORE INSERT OR UPDATE OF archived_at ON package_items
    FOR EACH ROW EXECUTE FUNCTION check_gear_ready();

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
  JOIN workflow_state_usage cur ON cur.workflow_id = v.workflow_id AND cur.state_id = v.state_id
  JOIN workflow_state_usage nxt ON nxt.workflow_id = v.workflow_id AND nxt.sort_order >= cur.sort_order
                               AND nxt.done_before_finish IS NOT NULL
  JOIN workflow_states ws ON ws.pkid = nxt.state_id
 WHERE v.archived_at IS NULL
UNION ALL
SELECT v.calendar_id, a.video_id, 'appointment', a.pkid, a.name, a.starts_at, a.done_at
  FROM appointments a JOIN videos v ON v.pkid = a.video_id
 WHERE a.archived_at IS NULL AND v.archived_at IS NULL;

-- ===== Schedule problems: the conflicts report =====
-- Things a person should look at: a pinned appointment that ends later than its T-minus allows
-- (needed_before, so it also catches one left after the finish), an overdue milestone, a video
-- still in a stage whose deadline has passed, and an asset that is not ready by the time a video
-- needs it. For appointments still to come: a package holding fewer of a model than it needs,
-- packed gear with an open wear or damage report, and packed gear that is no longer Ready
-- (damaged, out for repair, waiting to be cleaned or restocked). Once a shoot has started, gear
-- not yet checked out; once it has ended, gear checked out but not checked back in. A person,
-- room or piece of gear booked twice cannot happen (see the exclusion constraints), so it
-- never shows here.
CREATE VIEW schedule_problems AS
SELECT v.calendar_id, v.pkid AS video_id, 'appointment_too_late'::text AS problem,
       a.pkid AS item_id, a.name AS label, v.finish_at - a.needed_before AS at
  FROM appointments a JOIN videos v ON v.pkid = a.video_id
 WHERE a.archived_at IS NULL AND v.archived_at IS NULL AND a.done_at IS NULL
   AND a.ends_at > v.finish_at - a.needed_before          -- misses its T-minus
UNION ALL
SELECT e.calendar_id, e.video_id, 'milestone_overdue', e.item_id, e.label, e.due_at
  FROM calendar_entries e
 WHERE e.kind = 'milestone' AND e.done_at IS NULL AND e.due_at < now()
UNION ALL
SELECT v.calendar_id, v.pkid, 'stage_overdue', cur.pkid, COALESCE(cur.label, ws.label),
       v.finish_at - cur.done_before_finish
  FROM videos v
  JOIN workflow_state_usage cur ON cur.workflow_id = v.workflow_id AND cur.state_id = v.state_id
  JOIN workflow_states ws ON ws.pkid = cur.state_id
 WHERE v.archived_at IS NULL AND v.finish_at - cur.done_before_finish < now()
UNION ALL
SELECT v.calendar_id, v.pkid, 'asset_late', a.pkid, a.name, v.finish_at - u.needed_before
  FROM (SELECT video_id, asset_id, needed_before FROM video_assets WHERE archived_at IS NULL
        UNION ALL
        SELECT s.video_id, sa.asset_id, sa.needed_before
          FROM segment_assets sa JOIN segments s ON s.pkid = sa.segment_id
         WHERE sa.archived_at IS NULL AND s.archived_at IS NULL) u
  JOIN videos v ON v.pkid = u.video_id
  JOIN assets a ON a.pkid = u.asset_id
  JOIN workflow_states ws ON ws.pkid = a.state_id
 WHERE v.archived_at IS NULL AND a.archived_at IS NULL
   AND ws.code NOT IN ('ready', 'approved', 'not_needed')
   AND v.finish_at - u.needed_before < now()
UNION ALL
SELECT v.calendar_id, v.pkid, 'package_short', pn.pkid,
       p.name || ': ' || m.name || ' ' || got.n || ' of ' || pn.quantity, a.starts_at
  FROM package_needs pn
  JOIN packages p          ON p.pkid = pn.package_id AND p.archived_at IS NULL
  JOIN appointments a      ON a.pkid = p.appointment_id AND a.archived_at IS NULL
  JOIN videos v            ON v.pkid = a.video_id AND v.archived_at IS NULL
  JOIN equipment_models m  ON m.pkid = pn.model_id
  CROSS JOIN LATERAL (SELECT CASE WHEN m.is_tagged             -- pieces packed, or count ticked
           THEN (SELECT count(*) FROM package_items pi JOIN equipment_items i ON i.pkid = pi.item_id
                  WHERE pi.package_id = p.pkid AND pi.archived_at IS NULL AND i.model_id = m.pkid)
           ELSE (SELECT COALESCE(sum(s.quantity), 0) FROM package_supplies s
                  WHERE s.package_id = p.pkid AND s.archived_at IS NULL AND s.model_id = m.pkid)
           END::int AS n) got
 WHERE pn.archived_at IS NULL AND a.ends_at > now() AND got.n < pn.quantity
UNION ALL
SELECT DISTINCT v.calendar_id, v.pkid, 'gear_has_issue', i.pkid,
       m.name || ' #' || i.asset_tag || ': ' || ei.description, a.starts_at
  FROM equipment_issues ei
  JOIN equipment_items i  ON i.pkid = ei.item_id
  JOIN equipment_models m ON m.pkid = i.model_id
  JOIN package_items pi   ON pi.item_id = i.pkid AND pi.archived_at IS NULL
  JOIN packages p         ON p.pkid = pi.package_id AND p.archived_at IS NULL
  JOIN appointments a     ON a.pkid = p.appointment_id AND a.archived_at IS NULL
  JOIN videos v           ON v.pkid = a.video_id AND v.archived_at IS NULL
 WHERE ei.resolved_at IS NULL AND ei.archived_at IS NULL AND a.ends_at > now()
UNION ALL
SELECT v.calendar_id, v.pkid, 'gear_not_ready', i.pkid,
       m.name || ' #' || i.asset_tag || ': ' || ws.label, a.starts_at
  FROM package_items pi
  JOIN equipment_items i  ON i.pkid = pi.item_id
  JOIN equipment_models m ON m.pkid = i.model_id
  JOIN workflow_states ws ON ws.pkid = i.state_id
  JOIN packages p         ON p.pkid = pi.package_id AND p.archived_at IS NULL
  JOIN appointments a     ON a.pkid = p.appointment_id AND a.archived_at IS NULL
  JOIN videos v           ON v.pkid = a.video_id AND v.archived_at IS NULL
 WHERE pi.archived_at IS NULL AND a.starts_at > now() AND ws.code NOT IN ('ready', 'in_use')
UNION ALL
SELECT v.calendar_id, v.pkid,
       CASE WHEN pi.checked_out_at IS NULL THEN 'gear_not_checked_out' ELSE 'gear_not_returned' END,
       i.pkid, m.name || ' #' || i.asset_tag || ': ' || a.name,
       CASE WHEN pi.checked_out_at IS NULL THEN a.starts_at ELSE a.ends_at END
  FROM package_items pi
  JOIN equipment_items i  ON i.pkid = pi.item_id
  JOIN equipment_models m ON m.pkid = i.model_id
  JOIN packages p         ON p.pkid = pi.package_id AND p.archived_at IS NULL
  JOIN appointments a     ON a.pkid = p.appointment_id AND a.archived_at IS NULL
  JOIN videos v           ON v.pkid = a.video_id AND v.archived_at IS NULL
 WHERE pi.archived_at IS NULL AND pi.checked_in_at IS NULL
   AND ((pi.checked_out_at IS NULL AND a.starts_at <= now() AND a.done_at IS NULL)
     OR (pi.checked_out_at IS NOT NULL AND a.ends_at < now()));

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
 WHERE sr.role IN ('admin', 'auditor')        -- a workflow editor gets no content access from this
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
CREATE INDEX ON appointments      (video_id)                  WHERE archived_at IS NULL;
CREATE INDEX ON appointment_resources (resource_id);
CREATE INDEX ON crew_members (user_id);
CREATE INDEX ON package_items (item_id);
CREATE INDEX ON equipment_contents (container_id);
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

**Stages have deadlines too.** Each video stage carries a T-minus, such as "Pre-production done 14 days before finish", so the calendar shows when every remaining step of a video is due and flags the ones running late. The deadlines belong to the workflow version, so a Short workflow can run on a tighter clock than long-form. The seed data holds first guesses for both, and changing them means publishing a new version, not a code change.

Fixed appointments, such as a shoot booked with a guest, are the exception: they stay pinned to their real time (see Appointments, crews and problems below).

### Series cadence and slots

A series can have a **cadence**: a cycle, such as a week, two weeks or a month, with one or more release times in it. "Weekly on Tuesday at 10:00" is one time in a 7-day cycle, "Tuesdays and Thursdays at 10:00" is two, and "every other Friday" is one time in a 14-day cycle. The cadence keeps its own time zone, so 10:00 stays 10:00 when the clocks change. When you create a series, or later, you ask for a number of **slots**, say 10, 20 or 30, and the app creates that many videos straight away: "Episode 1", "Episode 2" and so on, each in the Pitch stage with its finish date at the next release time. A slot is not a new kind of thing. It is an ordinary video, so it has a finish date from the start, its stages and milestones count back from it, and it shows on the calendar at once.

After that the slots are just rows. Moving one episode's finish date moves only that episode and everything tied to it; the screen also offers "move this and every later episode" for when the whole run slips (guess). Changing the cadence never rewrites existing slots; it only sets the dates of slots added later. A series template carries a default number of slots; the wizard asks for the cadence.

```sql
-- Run in the wizard's transaction after SET LOCAL TimeZone to the cadence's zone, so adding
-- days keeps the local clock time. Adds $4 slots to series $1, using format $2 and workflow $5
-- (both from the chosen video template), at the first release times on or after $3.
INSERT INTO videos (calendar_id, series_id, episode_number, working_title,
                    format_id, workflow_id, state_id, finish_at)
SELECT s.calendar_id, s.pkid, x.n, 'Episode ' || x.n, $2, $5,
       (SELECT state_id FROM workflow_state_usage WHERE workflow_id = $5 AND is_initial),
       x.finish_at
  FROM series s
  CROSS JOIN LATERAL (
      SELECT f.finish_at,
             COALESCE((SELECT max(episode_number) FROM videos WHERE series_id = s.pkid), 0)
               + row_number() OVER (ORDER BY f.finish_at) AS n
        FROM (SELECT c.starts_at + k * c.cycle + t.offset_in_cycle AS finish_at
                FROM series_cadences c
                JOIN cadence_times t ON t.cadence_id = c.pkid AND t.archived_at IS NULL
                CROSS JOIN LATERAL (SELECT greatest(0, floor(extract(epoch FROM $3 - c.starts_at)
                                       / extract(epoch FROM c.cycle))::int - 1) AS k0) z
                CROSS JOIN LATERAL generate_series(z.k0, z.k0 + $4 + 1) AS k
               WHERE c.series_id = s.pkid) f
       WHERE f.finish_at >= $3
       ORDER BY f.finish_at
       LIMIT $4) x
 WHERE s.pkid = $1;
```

New series are made with a wizard: pick a series template or start blank, name the series, choose the video template its episodes start from, set the cadence, the first finish date and the number of slots, then review the laid-out dates and confirm. On confirm, one transaction creates the series and every slot video, copying each slot's segments, asset links, milestones and publications from the chosen video template. That copy happens once, at creation; later edits to the template do not change the episodes, and more slots added later go through the same step.

### Appointments, crews and problems

Some things happen at a fixed time and must not move with the finish date: a shoot booked with a guest, a studio slot, an interview. These are **appointments**, kept apart from milestones. An appointment has a real start and end and stays put, while a milestone counts back from the finish.

- **Appointments still have a T-minus.** Each one says how long before the finish it must be over (`needed_before`), such as a shoot that must wrap 10 days before finish so editing has time. The appointment doesn't move, but its deadline does, so a finish date moved earlier can leave it too late.
- **Crews are made of people, and people are users.** A **crew** (Crew A) is a named team; membership is dated and stays inside each person's employment, like group membership. Booking a crew onto an appointment books each person on it at that moment, so everyone can see and report on their own appointments. A member marked as crew lead also sees every booking of the people on that crew.
- **Nobody can be in two time slots at once.** The database refuses any booking of a person whose time overlaps another of their bookings, even when they are on two crews. That covers moving an appointment into a clash and bringing back an archived one. A person also can't be booked outside their employment, so ending someone with bookings still ahead is refused until those bookings are archived.
- **Studios and rooms are resources,** booked the same way and with the same rule. Locations don't clash: a crew can work at several locations in a day, and a location can host several shoots, even at once. Crews and resources are company-wide, because they can work for several calendars.
- **The problems report** (`schedule_problems`) lists what needs a person's attention: an appointment that ends later than its T-minus allows (including one now after the finish), an overdue milestone, a video still in a stage whose deadline has passed, and an asset that isn't ready when a video needs it. Each row points to its video, so someone can move the appointment, the finish date, or both.

```sql
-- Book crew $2 onto appointment $1: everyone on the crew when the appointment starts.
-- The exclusion constraint refuses the whole insert if any member is already booked then.
INSERT INTO appointment_people (appointment_id, user_id, period, appointment_live, employment)
SELECT a.pkid, u.pkid, a.period, a.is_live, u.employment
  FROM appointments a
  JOIN crew_members cm ON cm.crew_id = $2
                      AND tstzrange(cm.starts_at, cm.ends_at, '[)') @> a.starts_at
  JOIN users u ON u.pkid = cm.user_id
 WHERE a.pkid = $1
ON CONFLICT (appointment_id, user_id) WHERE archived_at IS NULL DO NOTHING;
-- (skips only people already on this appointment; a clash elsewhere still fails the insert)
```

```sql
-- My appointments: the signed-in user's bookings from $2 on, in time order.
SELECT a.pkid, a.name, a.starts_at, a.ends_at, a.location, a.done_at
  FROM appointment_people ap
  JOIN appointments a ON a.pkid = ap.appointment_id
  JOIN videos v       ON v.pkid = a.video_id AND v.archived_at IS NULL
 WHERE ap.user_id = $1 AND ap.archived_at IS NULL AND a.archived_at IS NULL
   AND a.ends_at >= $2
 ORDER BY a.starts_at;
```

```sql
-- Crew lead's page: bookings of everyone on the crews user $1 leads right now, from $2 on.
SELECT DISTINCT u.display_name, a.pkid, a.name, a.starts_at, a.ends_at, a.location
  FROM crew_members lead
  JOIN crew_members cm       ON cm.crew_id = lead.crew_id
                            AND tstzrange(cm.starts_at, cm.ends_at, '[)') @> now()
  JOIN users u               ON u.pkid = cm.user_id
  JOIN appointment_people ap ON ap.user_id = cm.user_id AND ap.archived_at IS NULL
  JOIN appointments a        ON a.pkid = ap.appointment_id AND a.archived_at IS NULL
  JOIN videos v              ON v.pkid = a.video_id AND v.archived_at IS NULL
 WHERE lead.user_id = $1 AND lead.is_lead
   AND tstzrange(lead.starts_at, lead.ends_at, '[)') @> now()
   AND a.ends_at >= $2
 ORDER BY a.starts_at, u.display_name;
```

### Equipment and packages

Gear is tracked piece by piece, because there may be several of the same camera and each one wears out or breaks on its own.

- **Models and items.** A **model** is a kind of gear (Sony FX3); an **item** is one piece of it. The system gives every item its own asset tag number to print on a labeler and stick on the gear, so nobody has to find the maker's serial number (which can still be recorded, and can't repeat within a model). Tags are never reused, even after an item is retired.
- **Package templates are kit lists** for a kind of job, such as "Interview kit: 2 × Sony FX3, 1 × Rode NTG5". They name models, not serial numbers, so the same gear can be repacked into different packages for different kinds of work.
- **A package is one crate for one appointment.** Packing from a template copies its list into the package's needs, which can then be changed. A video template can bring its kits along, so a new video already lists the kits its shoots will need. Then real items are packed against it. The checklist lives on screen: ticking an item as it goes in the crate records when (and the audit log records who), so anyone can see how far packing has got. It also prints for the crate: each model, how many are needed, and the asset tags packed, with a box to tick.
- **Gear can't be in two places at once.** An item can't be in two packages whose appointments overlap in time. Moving an appointment into a clash or bringing back an archived package is refused too. Archiving a package or its appointment unpacks the gear.
- **History and damage.** Every package an item was in stays on record, so you can see where it went, and wear or damage is logged as an issue on the item. The problems report flags a package that is short of what it needs, and packed gear with an open issue, for appointments still to come.

* **Check out and check in.** Every packed item is scanned out by its asset tag when the crate leaves for the shoot and scanned back in afterwards; the database won't let an item be checked out before it is packed, or checked in before it was checked out, and the audit log records who did each. The problems report flags gear not yet checked out once a shoot has started, and gear checked out but not back once it has ended, naming the shoot, so nothing goes missing without a trace. An item that doesn't come back is moved to Missing. Scanning a crate or case covers everything in it, so check-in shows its contents ticked and the person unticks whatever isn't there.

- **Gear has a workflow.** Each item is Ready, Out on a job, Needs cleaning, Needs restocking (reusables replaced), Damaged, or Out for repair, and moves only along the allowed steps, just like videos. Damage is flagged by moving the item to Damaged and logging what happened as an issue; gear that doesn't come back from a shoot goes to Missing until it is found. The gear workflow is versioned and editable like the others.
- **Only Ready gear can be packed.** The database refuses to pack an item in any other state, so a kit can't be booked while any of its gear is damaged, dirty or out for repair. Gear that breaks after it was packed stays packed and shows on the problems report as not ready.
- **Kits are persistent.** A **kit** is a real crate, such as Interview kit A, that keeps its gear between jobs. It names the kit list it should hold. The crate is itself a tagged item, and the kit is whatever is inside it now, cases and all; moving gear to another crate is a repack with a history. Booking a kit for an appointment makes a package from it in one step: the kit list becomes the needs, and the crate with everything in it is packed.

* **Accessories and nesting.** Gear has all sorts of bits that go with it. Every model has a **kind**: crate, case or bag, gear, accessory, or consumable. Tagged pieces can sit inside or attach to another piece: a battery and lens on a camera, the camera in its bag, the bag in a crate. Each piece is in one place at a time, and the record is dated, so its history shows. Each kind has a rank, and a piece can only go into one of lower rank (a crate holds a case, a case holds a camera, a camera holds its battery). Ranks only go up along a chain, so the database can't record anything inside itself, however deep, without needing a trigger. Packing a case packs everything in it, each piece still booked, checked out and missed on its own; scanning the case covers them all at once.
* **Consumables are counted.** Batteries, tape and gels have no asset tag. They are lines on the kit list (4 × AA) that are ticked with how many went in. They are used up, so they are not checked back in; the crate goes to Needs restocking instead. The database refuses a tagged model as a consumable line, and refuses an asset tag on a consumable.

```sql
-- Pack from template: copy package template $2's kit list into new package $1.
INSERT INTO package_needs (package_id, model_id, quantity)
SELECT $1, model_id, quantity
  FROM package_template_items
 WHERE package_template_id = $2 AND archived_at IS NULL;
```

```sql
-- Crate checklist for package $1, on screen or printed: each needed model with how many, then
-- the asset tags packed against it and what each sits in, so a case's contents print under it.
-- Gear packed beyond the list shows with needed = 0. Consumables show the count ticked.
SELECT m.name AS model, COALESCE(n.quantity, 0) AS needed,
       i.asset_tag, i.nickname, inside.asset_tag AS inside_tag, pi.packed_at,
       pi.checked_out_at, pi.checked_in_at, s.quantity AS supplies_packed
  FROM (SELECT model_id, quantity FROM package_needs WHERE package_id = $1 AND archived_at IS NULL) n
  FULL JOIN (SELECT pi.*, i.model_id FROM package_items pi
               JOIN equipment_items i ON i.pkid = pi.item_id
              WHERE pi.package_id = $1 AND pi.archived_at IS NULL) pi ON pi.model_id = n.model_id
  JOIN equipment_models m ON m.pkid = COALESCE(n.model_id, pi.model_id)
  LEFT JOIN equipment_items i ON i.pkid = pi.item_id
  LEFT JOIN equipment_contents c ON c.item_id = i.pkid AND c.added_at <= now()
                                AND (c.removed_at IS NULL OR c.removed_at > now())
  LEFT JOIN equipment_items inside ON inside.pkid = c.container_id
  LEFT JOIN package_supplies s ON s.package_id = $1 AND s.model_id = m.pkid AND s.archived_at IS NULL
 ORDER BY m.name, i.asset_tag;
```

```sql
-- Book kit $2 onto new package $1, in one statement: record the booking, copy the kit list into
-- the package's needs, and pack the crate with everything inside it now, cases and all. If any
-- piece is not Ready, the trigger refuses the whole statement.
WITH RECURSIVE b AS (
    INSERT INTO kit_bookings (package_id, kit_id) VALUES ($1, $2) RETURNING package_id, kit_id
), n AS (
    INSERT INTO package_needs (package_id, model_id, quantity)
    SELECT b.package_id, ti.model_id, ti.quantity
      FROM b JOIN kits k ON k.pkid = b.kit_id
      JOIN package_template_items ti ON ti.package_template_id = k.package_template_id
                                    AND ti.archived_at IS NULL
), tree AS (
    SELECT k.container_id AS item_id FROM b JOIN kits k ON k.pkid = b.kit_id
    UNION ALL
    SELECT c.item_id FROM tree t
      JOIN equipment_contents c ON c.container_id = t.item_id AND c.added_at <= now()
                               AND (c.removed_at IS NULL OR c.removed_at > now())
)
INSERT INTO package_items (package_id, item_id, period, package_live)
SELECT p.pkid, t.item_id, p.period, p.is_live
  FROM b JOIN packages p ON p.pkid = b.package_id
 CROSS JOIN tree t;
```

```sql
-- Pack item $2 into package $1 with everything inside or attached to it now: a case brings its
-- gear, a camera brings its battery and lens.
WITH RECURSIVE tree AS (
    SELECT $2::uuid AS item_id
    UNION ALL
    SELECT c.item_id FROM tree t
      JOIN equipment_contents c ON c.container_id = t.item_id AND c.added_at <= now()
                               AND (c.removed_at IS NULL OR c.removed_at > now())
)
INSERT INTO package_items (package_id, item_id, period, package_live, packed_at)
SELECT p.pkid, t.item_id, p.period, p.is_live, now()
  FROM packages p CROSS JOIN tree t
 WHERE p.pkid = $1;
```

```sql
-- Scan case $2 back in for package $1: it and its contents are checked in in one go, except the
-- asset tags in $3 that the person unticked as not there. Those stay out and go on the report.
-- Checking out is the same statement with checked_out_at.
WITH RECURSIVE tree AS (
    SELECT $2::uuid AS item_id
    UNION ALL
    SELECT c.item_id FROM tree t
      JOIN equipment_contents c ON c.container_id = t.item_id AND c.added_at <= now()
                               AND (c.removed_at IS NULL OR c.removed_at > now())
)
UPDATE package_items pi SET checked_in_at = now()
  FROM tree t JOIN equipment_items i ON i.pkid = t.item_id
 WHERE pi.package_id = $1 AND pi.item_id = t.item_id AND pi.archived_at IS NULL
   AND pi.checked_out_at IS NOT NULL AND pi.checked_in_at IS NULL
   AND i.asset_tag <> ALL ($3::bigint[]);
```

```sql
-- Where has item $1 been? Its packing history, newest first, with each appointment and video.
SELECT a.starts_at, a.ends_at, a.name AS appointment, a.location, v.working_title,
       p.name AS package, pi.checked_out_at, pi.checked_in_at
  FROM package_items pi
  JOIN packages p     ON p.pkid = pi.package_id
  JOIN appointments a ON a.pkid = p.appointment_id
  JOIN videos v       ON v.pkid = a.video_id
 WHERE pi.item_id = $1
 ORDER BY a.starts_at DESC;
```

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

## Building the screens

Most screens are the same idea again and again: a list, a form to add or change a row, and an archive button. They should come from shared code, not be written by hand each time.

- **Look for a crate first.** At a first look, the Rust admin crates for axum (such as `axum-admin`, `adminx` and `rustango`) are young or bring their own data layer, sign-in or JavaScript, and none of them would run our rules for us: the access check against `effective_access`, the audit transaction that sets `app.user_id`, `app.ip` and `app.request_id`, CSRF on every POST, and archive instead of delete. Worth a closer look before we write our own, but probably not a fit.
- **Otherwise, one shared CRUD module.** A Rust trait describes a table once: its fields, labels, which lookup each foreign key picks from, its object type for access, and its `sqlx::query!` statements. Generic axum handlers then give list, view, new, edit and archive pages for any table that implements it, and shared Askama templates draw them with Pico.css, using HTMX for inline edits and for ticking checklist items. Every write goes through one helper that checks access, opens the audit transaction and handles CSRF, so no screen can skip those.
- **Hand-written screens only where the work is special,** such as the calendar, the series wizard, the packing checklist and the problems report.

## Proposed CLAUDE.md changes

Rule 4 in the repo's CLAUDE.md says every query must filter by `user_id`. Shared ownership breaks that rule, so it needs replacing before any of this is built. Proposed wording:

```markdown
4. **Multiuser Security:** Content belongs to the company, not a user. Every query for
   content MUST (a) check the `AuthSession` user's employment covers now(), and (b) require the needed level from
   `effective_access` for that object type on its calendar or channel: `read` to view,
   `edit` to create, change or delete. Never take a user, calendar or channel
   id from the request as proof of access. Admin-only actions (calendars, channels, groups,
   roles, grants) need a current admin assignment; building or publishing workflows needs a
   current admin or workflow_editor assignment. Reading the audit log needs admin,
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

* **Appointments stay put.** A booked shoot or studio slot keeps its time when the finish date moves. Give it a T-minus (for example, wrap 10 days before finish); if it ends later than that, it shows on the problems report.
* **A crew can't be in two time slots at once.** Booking a person (directly or through a crew), a studio or a piece of gear that is already booked at an overlapping time is refused, even when the person is on two crews. Archive the other booking first, or pick another time. Two shoots at the same location are fine.
* **My appointments.** Everyone sees their own bookings, with time and place, on one page. Crew leads also see every booking of their crew.
* **Ending someone with bookings ahead.** Archive or reassign their future bookings first; the database won't let a booking sit outside someone's employment.
* **Packing for a shoot.** Pick a package template, adjust the list, then pack items by their asset tags, ticking each one off on screen as it goes in (or print the checklist for the crate). New gear gets its asset tag when it is added; print the label and stick it on. Gear already packed for an overlapping appointment is refused; log any wear or damage on the item.
* **Check out, check in.** Scan each item's tag as the crate leaves for a shoot and again when it comes back. Anything not back shows on the problems report with the shoot it went to; mark it Missing until it turns up.
* **Gear states.** After a job, mark each item Back (all good), Needs cleaning or Needs restocking; report damage to move it to Damaged, then Out for repair. Only Ready gear can be packed.
* **Kits stay packed.** Book a whole kit to a shoot in one step, cases and all. If anything in it isn't Ready, swap it for a Ready item (a repack) or fix it first.
* **Cases, accessories, consumables.** Put batteries and lenses on their camera, the camera in its bag and the bag in its crate; then packing or scanning the outer one brings everything inside along. Tick consumables with how many went in, and restock the crate when it comes back.

- **Workflows have versions.** Pick a workflow from the list when you create a video; a template usually picks it for you. Changing a workflow makes a new version. Videos already under way stay on the version they started with unless an admin moves them, and a move never changes their current stage.

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
- [ ] **Q29. Fixed appointments.** Answered: appointments stay pinned but carry a T-minus, the problems report flags any that end later than it allows, and crews and other resources can never be in two overlapping time slots.
- [ ] **Q30. One workflow, or several?** Answered: several. Workflows are named and versioned, and you pick one from a list. The seed data includes a Short workflow for videos.
- [ ] **Q31. Should a series remember its template?** Answered: the new-series wizard creates the episodes from a template video at creation time, and nothing links them afterwards.

* [ ] **Q32. Stage deadlines per format?** Answered by versioned workflows: deadlines belong to each workflow, so Shorts use a Short workflow with its own clock, and a video template names its workflow.
* [ ] **Q33. Cadences that one interval can't say.** Answered: yes. A cadence is a cycle with any number of release times, so Tuesdays and Thursdays is one cadence. "First Monday of the month" still doesn't fit; move those slots by hand.

- [ ] **Q34. Who builds and publishes workflows?** Answered: a role. Admins can give people the workflow editor role; everyone else picks from the list.
- [ ] **Q35. Workflows per calendar?** Answered: one list for the whole company.

* [ ] **Q36. Who manages crews, rooms and equipment?** \[Admins, like the workflow list; it could become its own role.\]
* [ ] **Q37. Warn ahead, and tell people?** The problems report shows what is already wrong. Should it also warn a few days ahead, and should it email the people involved? \[Report only, for now.\]
* [ ] **Q38. What can crew members see?** Answered: crew members see only their own bookings; a crew lead sees every booking of the people on the crews they lead. Anything more about the video still needs read access to it.
* [ ] **Q39. Should video templates carry appointments and kit lists,** so a new episode comes with its shoot and package templates ready? Answered: video templates carry kits, copied to each new video. Shoots themselves aren't templated yet.
* [ ] **Q40. Should gear with an open issue be blocked from packing?** Answered: gear has its own workflow (Ready, Out on a job, Needs cleaning, Needs restocking, Damaged, Out for repair), only Ready gear can be packed, and kits are persistent crates.
* [ ] **Q41. Count consumables back in?** Should leftover batteries and tape be counted when a crate comes back, to track what was used? \[No, for now: restocking the crate covers it.\]
