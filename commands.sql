-- database: c:\GitHub Repos\opass--hybrid-api\tolls.db3

-- Use the ▷ button in the top right corner to run the entire file.

SELECT s.name FROM "enters" e, "stations" s WHERE e.name_id == s.id AND e.direction_id = 1;
