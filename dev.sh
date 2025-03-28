cargo build
cd dev
docker build -t rustify-dev-database .
docker run -d --name rustify-database -p 5432:5432 rustify-dev-database
cd ..
sqlx migrate run