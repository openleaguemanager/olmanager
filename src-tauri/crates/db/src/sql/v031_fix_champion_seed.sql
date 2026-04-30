-- V31: Fix champion counterpicks/synergies data
-- Previous seed stored ALL counterpicks/synergies in EVERY champion.
-- This migration clears the champion table data so it can be reseeded correctly.
-- The application-level seed function (seed_from_json) will re-run on next game load
-- because it checks if the table is empty.

DELETE FROM champions;
