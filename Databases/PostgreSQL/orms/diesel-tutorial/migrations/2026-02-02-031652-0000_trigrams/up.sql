CREATE TABLE IF NOT EXISTS sentences
(
    id   serial PRIMARY KEY,
    text text NOT NULL
);

INSERT INTO sentences (text)
VALUES ($$My vacuum cleaner started making a weird noise, so I put it in the garage to think about what it’s done.$$),
       ($$I told my doctor I broke my arm in two places, and he told me to stop going to those places.$$),
       ($$My bank account balance is a constant reminder that I’m much better at spending money than I am at having it.$$),
       ($$I don’t need a hair dryer; I just stand outside and let the judgmental whispers of my neighbors dry me off.$$),
       ($$If you see me talking to myself, just move along—we’re having a team meeting and things are getting heated.$$),
       ($$My bed is a magical place where I suddenly remember everything I forgot to do today.$$),
       ($$I’m not saying I’m lazy, but I once ordered a pizza just so the delivery guy could let my cat out.$$),
       ($$Parallel lines have so much in common; it’s a shame they’ll never meet.$$),
       ($$I followed my heart, and it led me straight to the refrigerator.$$),
       ($$My superpower is the ability to walk into a room and instantly forget why I’m there.$$),
       ($$I finally realized that "adulting" is just walking around wondering when the real adults are going to show up.$$),
       ($$Every time I think about exercising, I lie down until the feeling passes.$$),
       ($$I’m on a whiskey diet; I’ve lost three days already.$$),
       ($$My houseplants aren't dead; they're just participating in a very long, very dry performance art piece.$$),
       ($$I put the "pro" in procrastination, but I'll tell you more about that tomorrow.$$)
;

CREATE EXTENSION IF NOT EXISTS pg_trgm;
CREATE INDEX IF NOT EXISTS sentences_text_trgm_idx ON sentences USING gin (text gin_trgm_ops);

