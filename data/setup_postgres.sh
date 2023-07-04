#! /bin/sh

sudo apt update
sudo apt install postgresql -y

sudo sed -i "s/#listen_addresses = 'localhost'/listen_addresses = '*'/" /etc/postgresql/12/main/postgresql.conf
sudo ufw allow 5432

sudo su - postgres << EOF
psql -c 'create database testdb1;'
psql -d testdb1 -c 'create table if not exists users (id serial not null, username varchar(256) not null , password varchar(256) not null, primary key (id));'
psql -c "ALTER USER postgres PASSWORD 'postgres';"
psql -c "CREATE USER vagrant WITH PASSWORD 'vagrant';"
EOF

echo "host    all             vagrant         127.0.0.1/32            md5" | sudo tee -a /etc/postgresql/12/main/pg_hba.conf
echo "host    all             all         0.0.0.0/0            md5" | sudo tee -a /etc/postgresql/12/main/pg_hba.conf

sudo service postgresql restart