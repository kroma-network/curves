use ark_ec::{
    models::CurveConfig,
    short_weierstrass::{Affine, Projective, SWCurveConfig},
    AffineRepr, CurveGroup,
};
use ark_ff::{Field, MontFp};
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
use ark_std::vec::Vec;

use crate::{Fq, Fq3, Fr};

pub type G2Affine = Affine<Parameters>;
pub type G2Projective = Projective<Parameters>;

#[derive(Clone, Debug, PartialEq, Eq, CanonicalSerialize, CanonicalDeserialize)]
pub struct G2Prepared(pub G2Affine);

impl From<G2Affine> for G2Prepared {
    fn from(other: G2Affine) -> Self {
        G2Prepared(other)
    }
}

impl From<G2Projective> for G2Prepared {
    fn from(q: G2Projective) -> Self {
        q.into_affine().into()
    }
}

impl<'a> From<&'a G2Affine> for G2Prepared {
    fn from(other: &'a G2Affine) -> Self {
        G2Prepared(*other)
    }
}

impl<'a> From<&'a G2Projective> for G2Prepared {
    fn from(q: &'a G2Projective) -> Self {
        q.into_affine().into()
    }
}

impl G2Prepared {
    pub fn is_zero(&self) -> bool {
        self.0.is_identity()
    }
}

impl Default for G2Prepared {
    fn default() -> Self {
        G2Prepared(G2Affine::generator())
    }
}

#[derive(Clone, Default, PartialEq, Eq)]
pub struct Parameters;

impl CurveConfig for Parameters {
    type BaseField = Fq3;
    type ScalarField = Fr;

    /// COFACTOR =
    /// 43276679045916726782882096851503554444292580777869919574700824986947162516693702667493938255647666346010819253090121562084993205202476199057555142869892665220155573207800985012241638987472334344174208389303164492698303448192856551557283997344470334833850065978668184377503856699635686872344035470027430053642178229054516302338812152178131995800255516474185251732445975837621097393375441662426280154371264547168198834382681059556891327702516519955053315674076980350109237328216856859758931256208439575383786363605925879337208599843910819433766160937121108797819223653884174994325142959644019600
    #[rustfmt::skip]
    const COFACTOR: &'static [u64] = &[
        0x4b77fca151d50b90,
        0x8c98a12bd486d2fb,
        0x1f0c9a51593693f8,
        0x1d6f388069c063c1,
        0x556e918748f06793,
        0x2cea7dc01aae2140,
        0x4216f0595cee44d0,
        0x7a5e400154f633cf,
        0xbb74eb9b6630846b,
        0x8eb48c92998f3358,
        0xbedd37f629e8e634,
        0xc541018fe4d10cc7,
        0x574956a099ace2c3,
        0xa597504275948226,
        0x7ecaaf050acb91f3,
        0x0f25b044f4e9c932,
        0xf8c39cbf0df97780,
        0xd8f9eda95d6abf3e,
        0xd1d80da227dd39c1,
        0x8b589c61531dbce7,
        0xfee4439281455474,
        0x9eea59baa2aeb4a1,
        0xa3b8a42c4e1e6f5a,
        0xc4b99b0d9b077d21,
        0xd09033887d09b4d2,
        0x4a86d8ebb7fdf52a,
        0xbe7ce44dd084e05d,
        0x4ed25f7ebe6c44b3,
        0xd7f8e3ef00255961,
        0xa1ad2ad61580ef78,
        0x19e70d3618ca3,
    ];

    /// COFACTOR^(-1) mod r =
    /// 45586359457219724873147353901735745013467692594291916855200979604570630929674383405372210802279573887880950375598
    const COFACTOR_INV: Fr = MontFp!("45586359457219724873147353901735745013467692594291916855200979604570630929674383405372210802279573887880950375598");
}

impl SWCurveConfig for Parameters {
    /// COEFF_A = (0, 0, COEFF_A * TWIST^2) = (0, 0, 5)
    const COEFF_A: Fq3 = Fq3::new(Fq::ZERO, Fq::ZERO, MontFp!("5"));

    /// COEFF_B = (G1::COEFF_B * TWIST^3, 0, 0) =
    /// (7237353553714858194254855835825640240663090882935418626687402315497764195116318527743248304684159666286416318482685337633828994152723793439622384740540789612754127688659139509552568164770448654259255628317166934203899992395064470477612,
    /// 0, 0)
    const COEFF_B: Fq3 = Fq3::new(
        MontFp!("7237353553714858194254855835825640240663090882935418626687402315497764195116318527743248304684159666286416318482685337633828994152723793439622384740540789612754127688659139509552568164770448654259255628317166934203899992395064470477612"),
        Fq::ZERO,
        Fq::ZERO,
    );

    /// AFFINE_GENERATOR_COEFFS = (G2_GENERATOR_X, G2_GENERATOR_Y)
    const GENERATOR: G2Affine = G2Affine::new_unchecked(G2_GENERATOR_X, G2_GENERATOR_Y);
}

const G2_GENERATOR_X: Fq3 = Fq3::new(G2_GENERATOR_X_C0, G2_GENERATOR_X_C1, G2_GENERATOR_X_C2);
const G2_GENERATOR_Y: Fq3 = Fq3::new(G2_GENERATOR_Y_C0, G2_GENERATOR_Y_C1, G2_GENERATOR_Y_C2);

/// G2_GENERATOR_X_C0 =
/// 13426761183630949215425595811885033211332897733228446437546263564078445562454176776915160094418980045665397361295624472103734543457352048745726512354895954850428989867542989474136256025045975283415690491751906307188562464175510373683338
pub const G2_GENERATOR_X_C0: Fq = MontFp!("13426761183630949215425595811885033211332897733228446437546263564078445562454176776915160094418980045665397361295624472103734543457352048745726512354895954850428989867542989474136256025045975283415690491751906307188562464175510373683338");

/// G2_GENERATOR_X_C1 =
/// 20471601555918880743198170952645906008198510944268658573129351735028343217532386920456705632337352161031960990613816401042894531220068552819818037605513359562118363589199569321421558696125646867661360498323171027455638052943806292028610
pub const G2_GENERATOR_X_C1: Fq = MontFp!("20471601555918880743198170952645906008198510944268658573129351735028343217532386920456705632337352161031960990613816401042894531220068552819818037605513359562118363589199569321421558696125646867661360498323171027455638052943806292028610");

/// G2_GENERATOR_X_C2 =
/// 3905053196875761830053608605277158152930144841844497593936739534395003062685449846381431331169369910535935138116320442345524758217411779027270883193856999691582831339845600938304719916501940381093815781408183227875600753651697934495980
pub const G2_GENERATOR_X_C2: Fq = MontFp!("3905053196875761830053608605277158152930144841844497593936739534395003062685449846381431331169369910535935138116320442345524758217411779027270883193856999691582831339845600938304719916501940381093815781408183227875600753651697934495980");

/// G2_GENERATOR_Y_C0 =
/// 8567517639523571619872938228644013584947463594196306323477160496987712111576624702939472765993995586889532559039169098780892505598589581147768095093536988446010255611523736706017580686335404469207486594272103717837888228343074699140243
pub const G2_GENERATOR_Y_C0: Fq = MontFp!("8567517639523571619872938228644013584947463594196306323477160496987712111576624702939472765993995586889532559039169098780892505598589581147768095093536988446010255611523736706017580686335404469207486594272103717837888228343074699140243");

/// G2_GENERATOR_Y_C1 =
/// 3890537069205870914984502594450293167889863914413852788876350245583932846980126025043974070704295857226211547108005650399870458089721518559480870503159804530091559886149680718531004778697982910253701559194337987238111062202037698927752
pub const G2_GENERATOR_Y_C1: Fq = MontFp!("3890537069205870914984502594450293167889863914413852788876350245583932846980126025043974070704295857226211547108005650399870458089721518559480870503159804530091559886149680718531004778697982910253701559194337987238111062202037698927752");

/// G2_GENERATOR_Y_C2 =
/// 10936269922612615564271188303104593362724754284143779051599749016735041389483971486958818324356025479751246744831831158558101688599198721653921723013062333636402617118847009085485166284126970598561393411916461254016145116183331671450721
pub const G2_GENERATOR_Y_C2: Fq = MontFp!("10936269922612615564271188303104593362724754284143779051599749016735041389483971486958818324356025479751246744831831158558101688599198721653921723013062333636402617118847009085485166284126970598561393411916461254016145116183331671450721");
