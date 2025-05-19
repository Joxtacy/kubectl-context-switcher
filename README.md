# kubectl-namespace-switcher

Change namespace

```sh
kubectl config set-context --current --namespace=<namespace>
```

Check current namespace (returns empty string if not explicitly set)

```sh
kubectl config view --minify --output 'jsonpath={..namespace}'
```

List all namespaces, separated by whitespace

```sh
kubectl get namespaces -o jsonpath="{.items[*].metadata.name}"
```
