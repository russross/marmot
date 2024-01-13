
envs_envs_required = [
    'ENVS 1210', 'ENVS 1215',
    'ENVS 2210',
    'GEO 1110', 'GEO 1115',
    'GEOG 3600', 'GEOG 3605',
    'CHEM 1210', 'CHEM 1215',
    'CHEM 1220', 'CHEM 1225',
    'BIOL 1610', 'BIOL 1615',
    'MATH 1060',

    'ENVS 2700R',
    'ENVS 4910',
]

envs_envs_electives = [
    'ENVS 3920',
]

envs_geo_required = [
    'ENVS 1210', 'ENVS 1215',
    'ENVS 2210',
    'GEO 1110', 'GEO 1115',
    'GEOG 3600', 'GEOG 3605',
    'CHEM 1210', 'CHEM 1215',
    'CHEM 1220', 'CHEM 1225',
    'BIOL 1610', 'BIOL 1615',
    'MATH 1060',

    'GEO 1220', 'GEO 1225',
    'GEO 2700R',
]

envs_geo_electives = [
    'ENVS 3920',
]

# [course, prereqs, coreqs, rotation, number of sections in current]
details = {
    'BIOL 1610': [[], ['BIOL 1615'], ['FA', 'SP'], 2],
    'BIOL 1615': [[], ['BIOL 1610'], ['FA', 'SP'], 15],

    'CHEM 1210': [['MATH 1050'], ['CHEM 1215'], ['FA', 'SP'], 3],
    'CHEM 1215': [[], ['CHEM 1210'], ['FA', 'SP'], 7],
    'CHEM 1220': [['CHEM 1210'], ['CHEM 1225'], ['FA', 'SP'], 3],
    'CHEM 1225': [['CHEM 1215'], ['CHEM 1220'], ['FA', 'SP'], 6],

    'ENVS 1210': [[], ['ENVS 1215'], ['FA', 'SP'], 1],
    'ENVS 1215': [[], ['ENVS 1210'], ['FA', 'SP'], 2],
    'ENVS 2210': [['ENVS 1210', 'ENVS 1215', 'MATH 1050'], ['CHEM 1210', 'CHEM 1215'], ['SP'], 1],
    'ENVS 2700R': [['ENVS 1210', 'ENVS 1215'], [], ['SP'], 1],
    'ENVS 3920': [[], [], ['SP'], 1],
    'ENVS 4910': [[], [], ['SP'], 1],

    'GEO 1110': [[], ['GEO 1115'], ['FA', 'SP'], 1],
    'GEO 1115': [[], ['GEO 1110'], ['FA', 'SP'], 1],
    'GEO 1220': [['GEO 1110'], ['GEO 1225'], ['SP'], 1],
    'GEO 1225': [['GEO 1115'], ['GEO 1220'], ['SP'], 1],
    'GEO 2700R': [['GEO 1110', 'GEO 1115'], [], ['SP'], 1],
    'GEOG 3600': [[], ['GEOG 3605'], ['FA', 'SP'], 1],
    'GEOG 3605': [[], ['GEOG 3600'], ['FA', 'SP'], 1],

    'MATH 1060': [['MATH 1050'], [], ['FA', 'SP'], 2],
}
