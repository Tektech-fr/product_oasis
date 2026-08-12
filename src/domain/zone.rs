pub const GRID_COLS: usize = 7;
pub const GRID_ROWS: usize = 3;

pub const BOARD_SIZE_MM: (f32, f32) = (420.0, 295.0);
pub const BOARD_ASPECT: f32 = BOARD_SIZE_MM.0 / BOARD_SIZE_MM.1;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ZoneId {
    Amenagement(u8),
    ResidenceArtiste,
    Cuisine,
    Construction,
    Reserve,
    Cabane,
    Tente(u8),
    HygieneEtIntendance,
    Marche,
    Bois(u8),
    Champ(u8),
    FoyerCentral,
}

impl ZoneId {
    pub fn label(self) -> &'static str {
        match self {
            Self::Amenagement(_) => "Aménagement",
            Self::ResidenceArtiste => "Résidence d'artiste",
            Self::Cuisine => "Cuisine",
            Self::Construction => "Construction",
            Self::Reserve => "Réserve",
            Self::Cabane => "Cabane",
            Self::Tente(_) => "Tente",
            Self::HygieneEtIntendance => "Hygiène et intendance",
            Self::Marche => "Marché",
            Self::Bois(_) => "Bois",
            Self::Champ(_) => "Champ",
            Self::FoyerCentral => "Foyer central",
        }
    }
}

/// A zone's position on the abstract layout grid (columns/rows — not
/// pixels: turning this into on-screen rects is `ui::geom`'s job).
pub struct GridCell {
    pub zone: ZoneId,
    pub col: usize,
    pub row: usize,
    pub col_span: usize,
}

const LAYOUT: &[GridCell] = &[
    GridCell {
        zone: ZoneId::Amenagement(1),
        col: 0,
        row: 0,
        col_span: 1,
    },
    GridCell {
        zone: ZoneId::Amenagement(2),
        col: 1,
        row: 0,
        col_span: 1,
    },
    GridCell {
        zone: ZoneId::Amenagement(3),
        col: 2,
        row: 0,
        col_span: 1,
    },
    GridCell {
        zone: ZoneId::Amenagement(4),
        col: 3,
        row: 0,
        col_span: 1,
    },
    GridCell {
        zone: ZoneId::Amenagement(5),
        col: 4,
        row: 0,
        col_span: 1,
    },
    GridCell {
        zone: ZoneId::ResidenceArtiste,
        col: 5,
        row: 0,
        col_span: 1,
    },
    GridCell {
        zone: ZoneId::Cuisine,
        col: 6,
        row: 0,
        col_span: 1,
    },
    GridCell {
        zone: ZoneId::Construction,
        col: 0,
        row: 1,
        col_span: 1,
    },
    GridCell {
        zone: ZoneId::Reserve,
        col: 1,
        row: 1,
        col_span: 2,
    },
    GridCell {
        zone: ZoneId::Cabane,
        col: 3,
        row: 1,
        col_span: 1,
    },
    GridCell {
        zone: ZoneId::Tente(1),
        col: 4,
        row: 1,
        col_span: 1,
    },
    GridCell {
        zone: ZoneId::Tente(2),
        col: 5,
        row: 1,
        col_span: 1,
    },
    GridCell {
        zone: ZoneId::HygieneEtIntendance,
        col: 6,
        row: 1,
        col_span: 1,
    },
    GridCell {
        zone: ZoneId::Marche,
        col: 0,
        row: 2,
        col_span: 1,
    },
    GridCell {
        zone: ZoneId::Bois(1),
        col: 1,
        row: 2,
        col_span: 1,
    },
    GridCell {
        zone: ZoneId::Bois(2),
        col: 2,
        row: 2,
        col_span: 1,
    },
    GridCell {
        zone: ZoneId::Champ(1),
        col: 3,
        row: 2,
        col_span: 1,
    },
    GridCell {
        zone: ZoneId::Champ(2),
        col: 4,
        row: 2,
        col_span: 1,
    },
    GridCell {
        zone: ZoneId::Champ(3),
        col: 5,
        row: 2,
        col_span: 1,
    },
    GridCell {
        zone: ZoneId::FoyerCentral,
        col: 6,
        row: 2,
        col_span: 1,
    },
];

pub fn grid_cells() -> impl Iterator<Item = &'static GridCell> {
    LAYOUT.iter()
}
