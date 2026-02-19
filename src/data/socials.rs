pub struct Social {
    pub name: &'static str,
    pub url: &'static str,
    pub handle: &'static str,
}

pub const SOCIALS: &[Social] = &[
    Social {
        name: "GitHub",
        url: "https://github.com/aicheye",
        handle: "aicheye",
    },
    Social {
        name: "LinkedIn",
        url: "https://linkedin.com/in/syang07",
        handle: "syang07",
    },
    Social {
        name: "Twitter / X",
        url: "https://x.com/aicheye",
        handle: "aicheye",
    },
    Social {
        name: "Instagram",
        url: "https://instagram.com/seanyang_esports_gaming",
        handle: "seanyang_esports_gaming",
    },
    Social {
        name: "Letterboxd",
        url: "https://letterboxd.com/aicheye",
        handle: "aicheye",
    },
    Social {
        name: "Email",
        url: "mailto:sean@seanyang.me",
        handle: "sean@seanyang.me",
    },
];

pub const PRIMARY_EMAIL: &str = "sean@seanyang.me";
