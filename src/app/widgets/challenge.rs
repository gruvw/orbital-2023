use rand::{seq::SliceRandom, Rng};
use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph, Wrap},
};

use crate::app::{
    data::game::{Game, CHALLENGE_POINTS},
    App,
};

const SOLO_CHALLENGES: &'static [&'static str] = &[
    "[DUEL] Shifumi",
    "[DUEL] Tenir le plus longtemps sur un pied",
    "[DUEL] Celui qui saute le plus haut",
    "[DUEL] Celui qui fait le meilleur avion en papier (nécessite papier)",
    "[DUEL] Premier joueur à trouver un objet [rose, à poil, ...]",
    "[DUEL] Guerre des pouces",
    "[DUEL] Citer l'un après l'autre des [marques de voitures, Pokémon, …]. Le premier a ne pas trouver en 5 secondes perds",
    "[DUEL] Le premier à finir un verre d'eau (nécessite eau)",
    "[DUEL] Le premier joueur à prononcer faux «panier piano» perd",
    "[DUEL] Le joueur qui arrive à jongler le plus longtemps gagne (nécessite trucs pour jongler)",
    "[DUEL] Pile ou face",
    "[DUEL] Celui qui dessine le meilleur [chat, fruit, …] en 30s",
    "[DUEL] Le premier à toucher le pied de l'autre joueur",
    "[DUEL] Combat de regard",
    "[DUEL] Le premier joueur a compter juste le nombre de cases posées sur le plateau gagne",
    "[DUEL] Celui qui court le plus vite en sprint",
    "[DUEL] Celui qui fait le plus de pompes",
    "[DUEL] Celui qui imite le mieux une célébrité",
];

const TEAM_CHALLENGES: &'static [&'static str] = &[
    "[TEAM] L'équipe qui enlève le plus vite ses chaussettes",
    "[TEAM] L'équipe qui construit la plus haute tour en 1 minutes",
    "[TEAM] La première équipe a citer 3 [animaux verts, sports d'hiver, acteurs, …]",
    "[TEAM] La première équipe à tous lever un membre de son corps (bras, jambes)",
    "[TEAM] La première équipe à tous toucher le plafond",
    "[TEAM] La première équipe à entièrement sortir de la pièce",
    "[TEAM] La première équipe à ne plus toucher le sol",
    "[TEAM] La première équipe à faire un cercle en se tenant les bras",
    "[TEAM] La première équipe à tous se toucher les genoux",
    "[TEAM] La première équipe à réciter les deux premières lignes d'un poème",
    "[TEAM] Chaque équipe choisit 3 lettres différentes de l'alphabet, l'équipe qui trouve le plus de mots différents en 1 minute gagne",
];

impl Game {
    pub fn pick_challenge(&self) -> &'static str {
        let mut rng = rand::thread_rng();

        if let Some(for_ai) = self.for_ai.as_ref() {
            if let Some(against_ai) = self.against_ai.as_ref() {
                if for_ai.nb_players > 1 && against_ai.nb_players > 1 {
                    return match rng.gen_range(0..=1) {
                        0 => SOLO_CHALLENGES.choose(&mut rng).unwrap(),
                        1 => TEAM_CHALLENGES.choose(&mut rng).unwrap(),
                        _ => panic!("Should never happen!"),
                    };
                }
            }
        }

        return SOLO_CHALLENGES.choose(&mut rng).unwrap();
    }
}

impl App<'_> {
    pub fn draw_challenge<B: Backend>(
        &mut self,
        f: &mut tui::Frame<B>,
        rect: Rect,
        text: &'static str,
    ) {
        let marged = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(100)])
            .horizontal_margin(50)
            .vertical_margin(20)
            .split(rect)[0];

        // TODO key desc
        let col = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Length(3), // Introduction msg
                    Constraint::Length(1),
                    Constraint::Length(6), // Challenge description
                    Constraint::Length(1),
                    Constraint::Length(3), // Keys
                    Constraint::Min(0),
                ]
                .as_ref(),
            )
            .margin(1)
            .split(marged);

        let outer_block = Block::default()
            .title(" Challenge ! ")
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Magenta));

        f.render_widget(outer_block, marged);

        let desc_row = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(100)].as_ref())
            .horizontal_margin(5)
            .split(col[2])[0];

        let prompt_block = Block::default()
            .title(" Description ")
            .title_alignment(Alignment::Left)
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Yellow));

        f.render_widget(prompt_block, desc_row);

        let msg = Paragraph::new(vec![
            Spans::from(""),
            Spans::from("You have to do the following challenge (either as a team or in a duel)."),
            Spans::from(format!(
                "The winning team receives {} points !",
                CHALLENGE_POINTS
            )),
        ])
        .alignment(Alignment::Center);
        f.render_widget(msg, col[0]);

        let prompt_row = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(100)].as_ref())
            .vertical_margin(1)
            .horizontal_margin(2)
            .split(desc_row)[0];

        let prompt = Paragraph::new(vec![Spans::from(""), Spans::from(Spans::from(text))])
            .wrap(Wrap { trim: false })
            .alignment(Alignment::Center)
            .style(Style::default().add_modifier(Modifier::BOLD));

        f.render_widget(prompt, prompt_row);

        let key_style = Style::default()
            .add_modifier(Modifier::BOLD)
            .fg(Color::Rgb(138, 138, 138));

        let key = Paragraph::new(vec![
            Spans::from(vec![
                Span::styled("[Q]", key_style),
                Span::from(": Cancel Challenge"),
                Span::from("    "),
                Span::styled("[F]", key_style),
                Span::from(": Change Challenge"),
            ]),
            Spans::from(vec![
                Span::styled("  [1]", key_style),
                Span::from(": Team For AI won"),
                Span::from("     "),
                Span::styled("[0]", key_style),
                Span::from(": Team Against AI won"),
            ]),
        ])
        .alignment(Alignment::Center);

        f.render_widget(key, col[4]);
    }
}
