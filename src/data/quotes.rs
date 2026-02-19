pub struct Quote {
    pub text: &'static str,
    pub author: &'static str,
}

pub const QUOTES: &[Quote] = &[
    Quote {
        text: "If you can't fly then run, if you can't run then walk, if you can't walk then crawl, but whatever you do you have to keep moving forward.",
        author: "Martin Luther King Jr.",
    },
    Quote {
        text: "Only a life lived for others is a life worthwhile.",
        author: "Albert Einstein",
    },
    Quote {
        text: "If not us, who? If not now, when?",
        author: "John F. Kennedy",
    },
    Quote {
        text: "To the mind that is still, the whole universe surrenders.",
        author: "Lao Tzu",
    },
    Quote {
        text: "I am afraid. Not of life, or death, or nothingness, but of wasting it as if I had never been.",
        author: "Daniel Keyes",
    },
    Quote {
        text: "I thank whatever gods may be for my unconquerable soul.",
        author: "William Ernest Henley",
    },
    Quote {
        text: "This too shall pass.",
        author: "Persian adage",
    },
    Quote {
        text: "In the midst of winter, I found there was, within me, an invincible summer.",
        author: "Albert Camus",
    },
    Quote {
        text: "Do not go gentle into that good night. Rage, rage against the dying of the light.",
        author: "Dylan Thomas",
    },
    Quote {
        text: "Don't tell me the sky's the limit when there are footprints on the moon.",
        author: "Paul Brandt",
    },
    Quote {
        text: "And you ask, 'What if I fall?' Oh, but my darling, what if you fly?",
        author: "Erin Hanson",
    },
];
