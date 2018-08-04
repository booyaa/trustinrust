# Trust in Rust

An OpenFaas function written in Rust.

This was featured in a blog post I wrote: https://booyaa.wtf/2018/run-rust-in-openfaas/

## Usage

You want to edit `trustinrust.yml` and update the following values (other parts of the code have been omitted):

```yml
provider:
  gateway: http://127.0.0.1:31112

functions:
  shuffle:
    image: DOCKER_ID/shuffle
```

The `gateway` assumes you're running on port 8080, kubernetes runs on a different port. Also I found the containers couldn't find the local repository so adding your docker id to the `image` ensures it uploads to Docker instead.


```shell
faas-cli build -f trustinrust.yml
faas-cli push -f trustinrust.yml
faas-cli deploy -f trustinrust.yml
curl -d 'a,b,c' http://localhost:31112/function/trustinrust
```

## License

OpenFaaS is MIT licensed, so is this project.
