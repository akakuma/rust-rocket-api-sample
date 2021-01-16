
rust and rocket and diesel で使ったsample APIです

### 環境

rustの環境を整える
https://blog.akakuma73.com/0b6cfhrcq67ys7uqpj09

### docker-composeをで立ち上げる

```shell
docker-compose up -d
```

### setup

```shell
docker-compose run --rm rust diesel setup
```

### migration

```
docker-compose run --rm rust diesel migration run
```

### run

```shell
docker-compose run --rm rust cargo run
```
