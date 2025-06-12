mod ptable;

use csv::ReaderBuilder;
use ptable::TABLE;
use serde::Deserialize;

pub struct Element {
    name: String,
    symbol: Symbol,
    atomic_number: u32,
    atomic_mass: f32,
}

impl Element {
    pub fn from(string: &String) -> () /* Result<Element, String */ {
        #[derive(Debug, Deserialize)]
        #[serde(rename_all = "PascalCase")]
        struct ElementRecord {
            #[serde(rename = "AtomicNumber")]
            atomic_number: u32,
            #[serde(rename = "Element")]
            name: String,
            symbol: String,
            #[serde(rename = "AtomicMass")]
            mass: f32,
            #[serde(rename = "NumberofNeutrons")]
            neutrons: u32,
            #[serde(rename = "NumberofProtons")]
            protons: u32,
            #[serde(rename = "NumberofElectrons")]
            electrons: u32,
            period: u32,
            group: u32,
            // phase: Phase,
            // radioactive: bool,
            // natural: bool,
            // metal: bool,
            // nonmetal: bool,
            // metalloid: bool,
            // element_type: ElementType,
            #[serde(rename = "AtomicRadius")]
            radius: f32,
            electronegativity: f32,
            #[serde(rename = "FirstIonization")]
            first_ionization_energy: f32,
            density: f32,
            melting_point: f32,
            boiling_point: f32,
            #[serde(rename = "NumberOfIsotopes")]
            isotopes: u32,
            discoverer: String,
            #[serde(rename = "Year")]
            year_of_discovery: u32,
            #[serde(rename = "SpecificHeat")]
            specific_heat_capacity: f32,
            #[serde(rename = "NumberofShells")]
            shells: u32,
            #[serde(rename = "NumberofValence")]
            valence_electrons: u32,
        }
        // pub const TABLE: &'static str = "AtomicNumber,Element,Symbol,AtomicMass,NumberofNeutrons,NumberofProtons,NumberofElectrons,Period,Group,Phase,Radioactive,Natural,Metal,Nonmetal,Metalloid,Type,AtomicRadius,Electronegativity,FirstIonization,Density,MeltingPoint,BoilingPoint,NumberOfIsotopes,Discoverer,Year,SpecificHeat,NumberofShells,NumberofValence
        let mut rdr = ReaderBuilder::new().from_reader(TABLE.as_bytes());
        for result in rdr.deserialize::<ElementRecord>() {
            println!("Name: {}", result.unwrap().name);
        }
    }
}

fn main() {
    Element::from(&"test".to_string());
}
pub enum Symbol {
    H,
    He,
    Li,
    Be,
    B,
    C,
    N,
    O,
    F,
    Ne,
    Na,
    Mg,
    Al,
    Si,
    P,
    S,
    Cl,
    Ar,
    K,
    Ca,
    Sc,
    Ti,
    V,
    Cr,
    Mn,
    Fe,
    Co,
    Ni,
    Cu,
    Zn,
    Ga,
    Ge,
    As,
    Se,
    Br,
    Kr,
    Rb,
    Sr,
    Y,
    Zr,
    Nb,
    Mo,
    Tc,
    Ru,
    Rh,
    Pd,
    Ag,
    Cd,
    In,
    Sn,
    Sb,
    Te,
    I,
    Xe,
    Cs,
    Ba,
    La,
    Ce,
    Pr,
    Nd,
    Pm,
    Sm,
    Eu,
    Gd,
    Tb,
    Dy,
    Ho,
    Er,
    Tm,
    Yb,
    Lu,
    Hf,
    Ta,
    W,
    Re,
    Os,
    Ir,
    Pt,
    Au,
    Hg,
    Tl,
    Pb,
    Bi,
    Po,
    At,
    Rn,
    Fr,
    Ra,
    Ac,
    Th,
    Pa,
    U,
    Np,
    Pu,
    Am,
    Cm,
    Bk,
    Cf,
    Es,
    Fm,
    Md,
    No,
    Lr,
    Rf,
    Db,
    Sg,
    Bh,
    Hs,
    Mt,
    Ds,
    Rg,
    Cn,
    Nh,
    Fl,
    Mc,
    Lv,
    Ts,
    Og,
}
