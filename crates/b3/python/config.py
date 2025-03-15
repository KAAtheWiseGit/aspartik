import b3

# Init parameters
kappa_noncoding = b3.Parameter(2.0)
kappa_pos_1 = b3.Parameter(2.0)
kappa_pos_2 = b3.Parameter(2.0)
kappa_pos_3 = b3.Parameter(2.0)

birth_rate = b3.Parameter(1.0)

gamma_shape_pos_1 = b3.Parameter(1.0)

seqs = b3.DNA(file="primate-mtDNA.fasta")
noncoding = seqs[1, 458:659, 897:898]
pos_1 = seqs[2:457:3, 660:896:3]
pos_2 = seqs[3:457:3, 661:896:3]
pos_3 = seqs[3:458:3, 662:896:3]

tree = b3.Tree(taxa=seqs)

yule = b3.priors.CalibratedYuleModel(
    name="Calibrated Yule model",
    tree=tree,
    birth_rate=birth_rate,
)
yule_dist = b3.priors.Distribution(
    b3.distribution.Gamma(
        alpha=b3.Parameter(0.001),
        beta=b3.Parameter(1000.0),
    ),
    birth_rate,
    name="Calibrated Yule birth rate prior",
)
kappa_pos_1_prior = b3.priors.Distribution(
    b3.distribution.LogNormal(
        mean=b3.Parameter(1.0),
        std_dev=b3.Parameter(1.25),
    ),
    gamma_shape_pos_1,
    name="Kappa prior 1st pos",
)
priors = [yule, yule_dist, kappa_pos_1_prior]

sub_noncoding = b3.substitutions.HKY(
    kappa=kappa_noncoding,
    frequencies=noncoding.frequencies(),
)
sub_1 = b3.substitutions.HKY(
    kappa=kappa_pos_1,
    frequencies=pos_1.frequencies(),
)
sub_2 = b3.substitutions.HKY(
    kappa=kappa_pos_2,
    frequencies=pos_2.frequencies(),
)
sub_3 = b3.substitutions.HKY(
    kappa=kappa_pos_3,
    frequencies=pos_3.frequencies(),
)

likelihoods = [
    b3.Likelihood(noncoding, sub_noncoding),
    b3.Likelihood(pos_1, sub_1),
    b3.Likelihood(pos_2, sub_2),
    b3.Likelihood(pos_3, sub_3),
]

operators = [
    b3.operators.ParamScale(
        name="Kappa scaler noncoding",
        parameter=kappa_noncoding,
        factor=0.1,
        weight=0.1,
    ),
    b3.operators.TreeNarrowExchange(
        name="Tree narrow exchange",
        tree=tree,
        weight=15.0,
    )
]

loggers = [
    b3.loggers.StateLogger(file="b3.state", every=5000),
    b3.loggers.TreeLogger(file="primate.trees", every=1000),
    b3.loggers.JSONLogger(
        file="params.json",
        every=1000,
        params=[
            kappa_noncoding,
            kappa_pos_1,
            kappa_pos_2,
            kappa_pos_3,
            birth_rate,
        ],
        priors=[yule, kappa_pos_1_prior],
    ),
    b3.loggers.StdoutLogger(
        every=500,

        posterior=True,
        likelihood=True,
        prior=True,

        priors=[yule],
    ),
]

state = b3.State(
    tree=tree,
    parameters=[
        kappa_noncoding,
        kappa_pos_1,
        kappa_pos_2,
        kappa_pos_3,
        birth_rate,
        gamma_shape_pos_1,
    ],
    seed=4,
)

# or load the state from a previous run:
# state = b3.State.load("b3.state")

b3.mcmc.run(
    burnin=100_000,
    length=1_000_000,

    state=state,
    priors=priors,
    likelihoods=likelihoods,
    operators=operators,
)
