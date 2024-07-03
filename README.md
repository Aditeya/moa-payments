# MoA Payments

MoA - Money of Awesome

## Endpoints

### Users

#### Create User

```sh
curl -XPOST -H "Content-type: application/json" -d '{"email": "abc@xyz.com","password":"1234"}' 'http://localhost:8080/api/v1/users'
```

#### Get User

```sh
curl -XGET 'http://localhost:8080/api/v1/users'
curl -XGET 'http://localhost:8080/api/v1/users/{email}'
```

#### Update User

```sh
curl -XPUT -H "Content-type: application/json" -d '{"email": "a@a.com"}' 'http://localhost:8080/api/v1/users/id/1'
curl -XPUT -H "Content-type: application/json" -d '{"password": "hello"}' 'http://localhost:8080/api/v1/users/id/1'
curl -XPOST -H "Content-type: application/json" -d '{"email": "a@a.com","password":"hello"}' 'http://localhost:8080/api/v1/users/id/1'
```

#### Delete User

```sh
curl -XDELETE -H 'Content-type: application/json' -d '{"email":"a@a.com", "password":"hello"}' 'http://localhost:8080/api/v1/users'
```
