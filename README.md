# RusQ

RusQ is the fastest and simplest message queuing app built in Rust, accepting queues in-memory with HTTP requests and executing commands defined in YAML.

## Usage

The easiest way to start rusq is to put the following in compose.yml and run the `docker compose up rusq` command.

```yaml
services:
  rusq:
    image: takumi3488/rusq
    ports:
      - "8080:8080"
    environment:
      RUSQ_HOST: "0.0.0.0"
      RUSQ_PASSWORD: password
      RUSQ_REDIS_URL: redis://dragonfly/
    depends_on:
      - dragonfly

  # Redis compatible in-memory database available.
  dragonfly:
    image: docker.dragonflydb.io/dragonflydb/dragonfly
```

After starting, you can send and receive messages as follows.

```
# Add message "first message" to queue named "first_queue".
$ curl -X "POST" -H "Content-Type: application/json" -d '{"message": "first message"}' http://rusq:password@localhost:8080/first_queue
{"queue":"first_queue","message":"first message"}

# Check queue status
$ curl http://rusq:password@localhost:8080
{"queues":[{"name":"rusq:first_queue","total":1}]}

# Get message in first_queue (FIFO)
curl http://rusq:password@localhost:8080/first_queue
{"queue":"first_queue","message":"first message"}
```

## Unsupported

- Queues other than FIFO
- Set expiration date
- Use of non-Redis compatible DB
- Arm architecture CPU
