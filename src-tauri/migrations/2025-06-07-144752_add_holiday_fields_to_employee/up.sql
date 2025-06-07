-- Your SQL goes here

-- Add holiday fields to employee table
ALTER TABLE employee ADD COLUMN holiday_per_year INTEGER NOT NULL DEFAULT 14;
ALTER TABLE employee ADD COLUMN accumulated_holidays INTEGER NOT NULL DEFAULT 0;
