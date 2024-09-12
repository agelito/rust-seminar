use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use rand::{distributions::Alphanumeric, thread_rng, Rng};

pub struct Movie {
    pub id: String,
    pub title: String,
    pub director: String,
    pub release_year: i32,
}

pub struct CreateMovie {
    pub title: String,
    pub director: String,
    pub release_year: i32,
}

pub struct UpdateMovie {
    pub id: String,
    pub title: Option<String>,
    pub director: Option<String>,
    pub release_year: Option<i32>,
}

#[derive(Clone)]
pub struct MovieRepository {
    movies: Arc<RwLock<HashMap<String, Movie>>>,
}

impl MovieRepository {
    pub fn new() -> Self {
        MovieRepository {
            movies: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn create_movie(&mut self, create_movie: CreateMovie) -> String {
        let id: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(24)
            .map(char::from)
            .collect();

        let mut movies = self.movies.write().expect("couldn't acquire write lock");

        let movie = Movie {
            id: id.clone(),
            title: create_movie.title,
            director: create_movie.director,
            release_year: create_movie.release_year,
        };

        movies.insert(id.clone(), movie);

        id
    }

    pub fn get_movie(&self, id: &str) -> Option<Movie> {
        let movies = self.movies.read().expect("couldn't acquire read lock");

        // NOTE: Since we're not returning a reference the Movie has to be manually cloned.
        movies.get(id).map(|m| Movie {
            id: m.id.clone(),
            title: m.title.clone(),
            director: m.director.clone(),
            release_year: m.release_year,
        })
    }

    pub fn get_movies(&self) -> Vec<Movie> {
        let movies = self.movies.read().expect("couldn't acquire read lock");

        let result = movies
            .values()
            .into_iter()
            .map(|m| Movie {
                id: m.id.clone(),
                title: m.title.clone(),
                director: m.director.clone(),
                release_year: m.release_year,
            })
            .collect();

        result
    }

    pub fn delete_movie(&self, id: &str) -> Option<Movie> {
        let mut movies = self.movies.write().expect("couldn't acquire write lock");

        movies.remove(id).map(|m| Movie {
            id: m.id.clone(),
            title: m.title.clone(),
            director: m.director.clone(),
            release_year: m.release_year,
        })
    }

    pub fn update_movie(&self, update: UpdateMovie) -> Option<Movie> {
        let mut movies = self.movies.write().expect("couldn't acquire write lock");

        if let Some(movie) = movies.get(&update.id) {
            let updated = Movie {
                id: update.id.clone(),
                title: update.title.unwrap_or(movie.title.clone()),
                director: update.director.unwrap_or(movie.director.clone()),
                release_year: update.release_year.unwrap_or(movie.release_year),
            };

            movies.insert(update.id, updated)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::movie_repository::CreateMovie;

    use super::MovieRepository;

    #[test]
    fn test_create_movie() {
        // Arrange
        let mut sut = MovieRepository::new();

        let title = "The Shawshank Redemption".to_string();
        let director = "Frank Darabont".to_string();
        let release_year = 1994;

        let create_movie = CreateMovie {
            title: title.clone(),
            director: director.clone(),
            release_year,
        };

        // Act
        let id = sut.create_movie(create_movie);

        // Assert
        let movie = sut.get_movie(&id).unwrap();

        assert_eq!(movie.title, title);
        assert_eq!(movie.director, director);
        assert_eq!(movie.release_year, release_year);
    }
}
