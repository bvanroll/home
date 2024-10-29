# k8s_manifests

## info


ran on ubuntu server because i was lazy
depends on microk8s with ingress and localstorage(wrong name) plugin
also the nfs-common package (apt install nfs-common)

apart from that, plex claim key thing has to be made using kubectl create secret SECRETNAME --from-literal=KEYNAME=claim-...
(see plex/pod.yaml)

also don't forget to edit the nfs target etc.

## TODO
