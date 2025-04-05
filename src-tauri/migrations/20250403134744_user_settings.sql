CREATE TABLE IF NOT EXISTS user_settings (
    id INTEGER NOT NULL,
    theme TEXT NOT NULL,
    highlight_current_day BOOLEAN NOT NULL,
    show_category_filter BOOLEAN NOT NULL,
    default_timeline TEXT NOT NULL
);

INSERT INTO user_settings (id, theme, highlight_current_day, show_category_filter, default_timeline)
VALUES (1, 'dark', true, false, 'year');
