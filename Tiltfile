docker_build(
    'agabani/platform',
    context='.',
)

k8s_yaml(helm(
    'helm',
    set=[]
))

k8s_resource(
    'chart-platform',
    port_forwards=['8080:8080'],
)
