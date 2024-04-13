
Admin shouldn't use the app normally, he has different view, doesn't use requests normally, etc.

All load_all requests should include 'user_id: i32' (not Option, stop it)


Deployment:

docker build -t rust-calendar -f ./Dockerfile .

docker tag rust-calendar artemzaycev/rust-calendar

docker push artemzaycev/rust-calendar -a
