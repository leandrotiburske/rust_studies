# Greedy coin microservice

## To-do list

- Add more unit test cases for edge cases.

- Modify the algorithm to take absolute value for negative amounts.

- Change cent values to use a Decimal type instead of u32.

- Add logging using the Log crate and display in JSON.

- Add a healthcheck endpoint for the service.

- In your own local environment, run Docker 
using this project source code and deploy it.

```bash
docker build -t myimage .
docker run -dp 3000:3000 myimage #runs in background
#find image
docker ps
#kill image (from id you found)
#docker stop 0a0200f4e53b 
```