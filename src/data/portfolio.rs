pub struct Project {
    pub title: &'static str,
    pub year: u16,
    pub category: &'static str,
    pub description: &'static str,
    pub long_description: &'static str,
    pub languages: &'static [&'static str],
    pub features: &'static [&'static str],
    pub github: Option<&'static str>,
}

pub const PROJECTS: &[Project] = &[
    Project {
        title: "CRusTTY",
        year: 2026,
        category: "Developer Tool",
        description: "Custom C interpreter with step-through debugger TUI built with Rust and Ratatui.",
        long_description: "CRusTTY is a from-scratch C language interpreter paired with an interactive \
                           step-through debugger rendered entirely in the terminal using Ratatui. It lets \
                           you pause execution at any point, inspect memory, and trace variable state\
                           all without leaving the terminal.",
        languages: &["Rust"],
        features: &[
            "C language interpreter",
            "Step-through debugging",
            "TUI with Ratatui",
            "Interactive memory inspection",
        ],
        github: Some("https://github.com/aicheye/crustty"),
    },
    Project {
        title: "Wunder RNN Challenge",
        year: 2025,
        category: "Machine Learning",
        description: "Custom transformer model for time-series market data prediction. Ranked top 5%.",
        long_description: "Built a custom transformer architecture in PyTorch for predicting time-series \
                           financial market data as part of the Wunder RNN challenge. Achieved a top 5% \
                           ranking by designing attention mechanisms tailored to sequential price signals.",
        languages: &["Python"],
        features: &[
            "Custom transformer architecture",
            "Time-series prediction",
            "Top 5% ranking",
            "PyTorch",
        ],
        github: Some("https://github.com/aicheye/wundernn"),
    },
    Project {
        title: "Bucket",
        year: 2025,
        category: "Web Application",
        description: "All-in-one student dashboard for Waterloo students.",
        long_description: "Bucket is a unified dashboard for University of Waterloo students, aggregating \
                           timetables, grades, deadlines in one place. Built with TypeScript and Hasura \
                           GraphQL for real-time updates.",
        languages: &["TypeScript", "GraphQL", "Docker"],
        features: &[
            "Student dashboard",
            "Real-time data",
            "Hasura GraphQL",
            "Responsive design",
        ],
        github: Some("https://github.com/aicheye/bucket"),
    },
    Project {
        title: "CampusYap",
        year: 2025,
        category: "Mobile App",
        description: "Location-based social media for university students, built natively on Android.",
        long_description: "CampusYap lets university students post and browse content anchored to specific \
                           campus locations. Built with Kotlin for Android, featuring real-time messaging \
                           via WebSocket and a Python backend.",
        languages: &["Kotlin", "Python"],
        features: &[
            "Location-based feed",
            "Social networking",
            "Real-time messaging",
            "Android native",
        ],
        github: Some("https://github.com/aicheye/project_team_23"),
    },
    Project {
        title: "EZP2P Arcade",
        year: 2024,
        category: "Gaming Platform",
        description: "Peer-to-peer gaming platform with classic games: no server required.",
        long_description: "EZP2P Arcade uses WebRTC to connect players directly, eliminating the need for \
                           a game server. Multiple classic games are playable with anyone who has a browser, \
                           over a direct peer connection.",
        languages: &["React", "Vite", "TypeScript"],
        features: &[
            "P2P via WebRTC",
            "Multiple games",
            "No server required",
            "Shareable room links",
        ],
        github: Some("https://github.com/aicheye/ezp2p"),
    },
    Project {
        title: "Combat Tag",
        year: 2024,
        category: "Minecraft Mod",
        description: "Fair PvP Minecraft mod with 870+ downloads on Modrinth.",
        long_description: "A Minecraft Fabric mod that implements a configurable combat-tagging system to \
                           prevent combat logging in PvP servers. Downloaded over 870 times on Modrinth, \
                           with extensive configurability via a JSON config file.",
        languages: &["Java"],
        features: &[
            "Combat tag system",
            "Customizable config",
            "Performance optimized",
            "870+ downloads",
        ],
        github: Some("https://github.com/aicheye/combat-tag"),
    },
    Project {
        title: "seanyang.me",
        year: 2024,
        category: "Portfolio Website",
        description: "Personal portfolio built with Next.js 15, Tailwind CSS v4, and a Flask backend.",
        long_description: "The website you're (sort of) looking at right now. Built with Next.js 15 and \
                           React 19 for the frontend, Tailwind CSS v4 for styling, and a Python/Flask \
                           backend powering the poke API.",
        languages: &["JavaScript", "Python"],
        features: &[
            "Next.js 15 + React 19",
            "Tailwind CSS v4",
            "Interactive animations",
            "Flask backend",
        ],
        github: Some("https://github.com/aicheye/seanyang.me"),
    },
    Project {
        title: "Quoridor",
        year: 2023,
        category: "Game AI",
        description: "Minimax game agent for Quoridor with alpha-beta pruning and pathfinding.",
        long_description: "Implemented a minimax agent for the board game Quoridor in Java, applying \
                           alpha-beta pruning to efficiently search the game tree. Custom heuristics \
                           incorporate shortest-path distances computed via BFS.",
        languages: &["Java"],
        features: &[
            "Minimax algorithm",
            "Alpha-beta pruning",
            "BFS pathfinding",
            "Heuristic optimization",
        ],
        github: Some("https://github.com/aicheye/quoridor"),
    },
    Project {
        title: "Chess in Python",
        year: 2021,
        category: "Game",
        description: "Chess GUI with full move validation and Stockfish engine integration.",
        long_description: "A Python chess application with a pygame GUI, complete move validation, and \
                           integration with the Stockfish engine for AI opponent play. Supports full \
                           standard chess rules including en passant and castling.",
        languages: &["Python"],
        features: &[
            "pygame GUI",
            "Stockfish integration",
            "Full move validation",
            "En passant & castling",
        ],
        github: Some("https://github.com/aicheye/chess"),
    },
];

pub struct Building {
    pub name: &'static str,
    pub description: &'static str,
}

pub const BUILDING: &[Building] = &[Building {
    name: "Watonomous",
    description: "Rapid-inference motion prediction models for autonomous vehicles",
}];
