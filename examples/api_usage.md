# Maze API Usage Examples

## Server Setup

The server runs on `http://127.0.0.1:3000` with API version `v1`.

## API Endpoints

### 1. Create a Map
**POST** `/v1/maps`

Request body:
```json
{
    "map_string": "####\n#  #\n#  #\n####"
}
```

Response:
```json
{
    "data": {
        "id": "550e8400-e29b-41d4-a716-446655440000",
        "map_string": "####\n#  #\n#  #\n####"
    }
}
```

### 2. List All Maps
**GET** `/v1/maps`

Response:
```json
{
    "data": [
        {
            "id": "550e8400-e29b-41d4-a716-446655440000",
            "map_string": "####\n#  #\n#  #\n####"
        }
    ]
}
```

### 3. Get Specific Map
**GET** `/v1/maps/{id}`

Response:
```json
{
    "data": {
        "id": "550e8400-e29b-41d4-a716-446655440000",
        "map_string": "####\n#  #\n#  #\n####"
    }
}
```

### 4. Delete a Map
**DELETE** `/v1/maps/{id}`

Response: 204 No Content

### 5. Solve a Map
**POST** `/v1/maps/{id}/solve`

Request body:
```json
{
    "start": {
        "x": 1,
        "y": 1
    },
    "end": {
        "x": 2,
        "y": 2
    }
}
```

Response:
```json
{
    "data": {
        "solution_map": "####\n#i #\n# .#\n####",
        "path_found": true
    }
}
```

## Map Format

- `#` - Wall
- ` ` (space) - Empty cell
- `i` - Start position (after solving)
- `O` - End position (after solving) 
- `.` - Path (after solving)

## Example Usage with curl

### Create a map:
```bash
curl -X POST http://127.0.0.1:3000/v1/maps \
  -H "Content-Type: application/json" \
  -d '{
    "map_string": "####\\n#  #\\n#  #\\n####"
  }'
```

### Solve a map:
```bash
curl -X POST http://127.0.0.1:3000/v1/maps/{MAP_ID}/solve \
  -H "Content-Type: application/json" \
  -d '{
    "start": {"x": 1, "y": 1},
    "end": {"x": 2, "y": 2}
  }'
```

### List all maps:
```bash
curl http://127.0.0.1:3000/v1/maps
```