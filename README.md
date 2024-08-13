# todo-app-api

## Tech Stack
- Rust (programming language)
- Actix Web (api framework)
- mold (a fast linker)
- Diesel (or mapper)
- MySQL (rdbms)

## LOG
```bash
# compile mold linker (aproox. 30 min.)
git clone --branch stable https://github.com/rui314/mold.git
cd mold
./install-build-deps.sh
cmake -DCMAKE_BUILD_TYPE=Release -DCMAKE_CXX_COMPILER=c++ -B build
cmake --build build -j$(nproc)
sudo cmake --build build --target install

cargo new todo-app-api
rustc --version
rustup toolchain list
rustup toolchain add 1.80.0  # diesel_cli requires rustc 1.78.0 or newer
# edit rust-toolchain.toml
# edit Cargo.toml
cargo install cargo-watch
# mold -run ...
# cargo watch -x ...
# cargo run
mold -run cargo watch -x run
# test
curl -X GET http://localhost:8080/
# also, use Postman
cargo clean

sudo apt update
sudo apt install libmysqlclient-dev
sudo mysql -u root
mysql> select user, host, plugin from mysql.user;
mysql> update mysql.user set plugin = 'caching_sha2_password' where user = 'root';
mysql> FLUSH PRIVILEGES;
mysql> ALTER USER 'root'@'localhost' IDENTIFIED BY 'password';
mysql> FLUSH PRIVILEGES;
mysql> quit;
mysql -u root -p

cargo install diesel_cli --no-default-features --features mysql
# edit .env (DATABASE_URL)
diesel setup
diesel migration generate create_tasks_table
# edit up.sql and down.sql
diesel migration run  # generate src/schema.rs
diesel print-schema
diesel migration redo
```

## References
- official
  - <https://github.com/rui314/mold>
  - <https://github.com/actix/actix-web>
  - <https://actix.rs/docs/getting-started>
  - <https://docs.rs/actix-web/latest/actix_web/>
  - <https://www.postman.com/>
  - <https://github.com/diesel-rs/diesel/tree/master/diesel_cli>
- overall
  - <https://github.com/flosse/rust-web-framework-comparison#server-frameworks>
  - <https://github.com/nemesiscodex/actix-todo>
  - <https://github.com/tetter27/webapi_mvp>
  - <https://ozway.jp/2020/10/rust-mysql-diesel%EF%BC%881%EF%BC%89/>
- spot
  - <https://redj.hatenablog.com/entry/2023/04/09/012242>
