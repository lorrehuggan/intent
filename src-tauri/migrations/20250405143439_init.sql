CREATE TABLE IF NOT EXISTS user_settings (
    id INTEGER NOT NULL,
    theme TEXT NOT NULL,
    highlight_current_day BOOLEAN NOT NULL,
    show_category_filter BOOLEAN NOT NULL,
    default_timeline TEXT NOT NULL
);

INSERT INTO user_settings (id, theme, highlight_current_day, show_category_filter, default_timeline)
VALUES (1, 'light', true, false, 'year');

CREATE TABLE IF NOT EXISTS habit_commit (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    habit_id INTEGER NOT NULL,
    note TEXT,
    image TEXT,
    created TEXT NOT NULL,
    updated TEXT,
    completion INTEGER NOT NULL,
    completed INTEGER NOT NULL CHECK (completed IN (0, 1)),
    completion_date TEXT,
    FOREIGN KEY (habit_id) REFERENCES habit(id)
);

CREATE TABLE IF NOT EXISTS habit (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title TEXT NOT NULL,
    description TEXT NOT NULL,
    status TEXT NOT NULL,           -- Enum: HabitStatus
    streak TEXT NOT NULL,           -- Enum: HabitStreak
    completions INTEGER NOT NULL,
    completions_needed INTEGER NOT NULL DEFAULT 1,
    icon TEXT,
    theme TEXT NOT NULL,            -- Enum: HabitTheme
    category TEXT NOT NULL,         -- Enum: HabitCategory
    reminder TEXT,                  -- Optional Enum: HabitReminder
    created TEXT NOT NULL,          -- ISO 8601 datetime
    updated TEXT                    -- Nullable
);

CREATE TABLE IF NOT EXISTS habit_stack (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    parent_habit_id INTEGER NOT NULL,    -- Triggers the next habit
    child_habit_id INTEGER NOT NULL,     -- Gets triggered
    position INTEGER NOT NULL DEFAULT 0, -- Optional: for ordering multiple children
    FOREIGN KEY (parent_habit_id) REFERENCES habit(id),
    FOREIGN KEY (child_habit_id) REFERENCES habit(id),
    UNIQUE (parent_habit_id, child_habit_id)  -- Prevent duplicates
);
