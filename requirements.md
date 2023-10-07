# Endpoints

The API should offer the following endpoints:

- List all songs: `GET /musicas`
    - Returns a list of all available songs.
- Get a song by ID: `GET /musicas/{id}`
    - Returns detailed information about a specific song based on its ID.
- Add a new song: `POST /musicas`
    - Allows the addition of a new song with information such as title, artist, genre, and duration.
- Update song information: `PUT /musicas/{id}`
    - Allows the update of specific song information based on its ID.
- Delete a song: `DELETE /musicas/{id}`
    - Allows the deletion of a specific song based on its ID.

# Data Fields

Each music record should contain the following fields:

- ID: A unique identifier for the song.
- Title: The title of the song.
- Artist: The name of the artist or band who performed the song.
- Album: The album in which the song is included (optional).
- Genre: The musical genre of the song.
- Duration: The duration of the song in minutes (e.g., 3.5 for 3 minutes and 30 seconds).
