# Greedy coin microservice

## Done

- Added more unit test cases for edge cases;

- Ran Docker using this project source code and deployeddocker it.

## To-do list

- Modify the algorithm to take absolute value for negative amounts.

- Change cent values to use a Decimal type instead of u32.

- Add logging using the Log crate and display in JSON.

- Add a healthcheck endpoint for the service.

```bash
docker build -t myimage .
docker run -dp 3000:3000 myimage #runs in background
#find image
docker ps
#kill image (from id you found)
#docker stop 0a0200f4e53b 
```