-- V32: Re-seed champions table with fixed name generation
-- Previous seed (v031) cleared counterpicks/synergies bug but names were still
-- generated with the old buggy camelCase logic that replaced first uppercase letter.
-- e.g., 'Taliyah' -> '. aliyah', 'Samira' -> '. amira'
-- This migration clears the table so seed_from_json re-runs with the fixed logic
-- that correctly handles camelCase: 'Taliyah' -> 'Taliyah', 'DrMundo' -> 'Dr. Mundo'

DELETE FROM champions;
