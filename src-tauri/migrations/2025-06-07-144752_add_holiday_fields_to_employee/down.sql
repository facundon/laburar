-- This file should undo anything in `up.sql`

-- Remove holiday fields from employee table
ALTER TABLE employee DROP COLUMN holiday_per_year;
ALTER TABLE employee DROP COLUMN accumulated_holidays;
