load('ext://helm_resource', 'helm_resource', 'helm_repo')

# Dgraph
helm_repo('dgraph', 'https://charts.dgraph.io',
    labels='dgraph',
)

helm_resource('dgraph-chart', 'dgraph/dgraph',
    release_name='dgraph',
    labels='dgraph',
    flags=[
        '--set', 'ratel.enabled=true',
    ],
    resource_deps=['dgraph'],
)

local_resource(
    'dgraph-alpha',
    serve_cmd='kubectl port-forward service/dgraph-dgraph-alpha 8080:8080',
    labels='dgraph',
    links='http://127.0.0.1:8080',
    resource_deps=['dgraph-chart'],
)

local_resource(
    'dgraph-ratel',
    serve_cmd='kubectl port-forward service/dgraph-dgraph-ratel 8000:80',
    labels='dgraph',
    links='http://127.0.0.1:8000',
    resource_deps=['dgraph-chart'],
)

local_resource(
    'dgraph-zero',
    serve_cmd='kubectl port-forward service/dgraph-dgraph-zero 6080:6080',
    labels='dgraph',
    links='http://127.0.0.1:6080',
    resource_deps=['dgraph-chart'],
)

# Platform
docker_build(
    'agabani/platform',
    context='.',
)

k8s_yaml(helm(
    'helm',
    name='platform',
    set=[],
))

k8s_resource(
    'platform',
    objects=[
        'platform:serviceaccount'
    ],
    labels='platform',
)

local_resource(
    'platform-api',
    serve_cmd='kubectl port-forward service/platform 4000:80',
    labels='platform',
    links='http://127.0.0.1:4000',
    resource_deps=['platform'],
)
