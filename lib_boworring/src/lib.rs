use inventory::{Book, Inventory};
use lib_users::{User, UserManager};

pub struct BorrowingService;

impl BorrowingService {
    pub fn new() -> Self {
        BorrowingService
    }

    pub fn borrow_book(
        &self,
        inventory: &mut Inventory,
        user_manager: &mut UserManager,
        user_id: u32,
        book_id: u32,
    ) -> Result<(), String> {
        // check if the book exist and is temporary available

        let book = inventory
            .get_book(book_id)
            .ok_or_else(|| "Book Inexistant".to_string())?;

        if !book.is_available {
            return Err(String::from("Book Temporary not Available"));
        }

        // check if the user exists or hasn't borrowed more than 2 books

        let user = user_manager
            .get_user(user_id)
            .ok_or_else(|| "User not found .".to_string())?;

        if user.borrowed_books.len() >= 3 {
            return Err(String::from(
                "User cannot borrow more than 2 books at a time .",
            ));
        }

        // update the book availability
        inventory.update_book_availability(book_id, false)?;

        // Add book to user's borrowed books
        user_manager.borrow_book(user_id, book_id);

        Ok(())
    }

    pub fn return_book(
        &self,
        inventory: &mut Inventory,
        user_manager: &mut UserManager,
        user_id: u32,
        book_id: u32,
    ) -> Result<(), String> {
        // Try to return the book in the user manager and check if it was successful
        user_manager.return_book(user_id, book_id)?;

        user_manager.return_book(user_id, book_id);

        // If successful in user manager, update the book's availability in the inventory
        inventory.update_book_availability(book_id, true)?;

        Ok(())
    }

        Ok(())
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use lib_users::{User, UserManager};
    use inventory::{Genre, Book};

    fn setup() -> (Inventory, UserManager, BorrowingService) {
        let mut inventory = Inventory::new();
        let mut user_manager = UserManager::new();
        let borrowing_service = BorrowingService::new();

        // Add a book 

        let book = Book {
            id: 1,
            title: String::from("Test Book"),
            author: String::from("Test Author"),
            genre: Genre::Manga,
            is_available: true,
        };

        inventory.add_book(book);

        // Add a user 

        let user = User {
            id: 1,
            name: String::from("Test User"),
            borrowed_books: Vec::new(),
        };

        user_manager.register_user(user);

       (inventory, user_manager, borrowing_service)
    }


    // Test borrow book and return book 
    // Borrow unavailable book 

    #[test]
    fn test_successful_borrow_and_return_book () {
        let (mut inventory, mut user_manager, borrowing_service) = setup();

        // Borrow the book 
        assert!(borrowing_service.borrow_book(&mut inventory, &mut user_manager, 1, 1).is_ok());
        assert!(!inventory.get_book(1).unwrap().is_available);
        assert!(user_manager.get_user(1).unwrap().borrowed_books.contains(&1));

        // Return the book 
        assert!(borrowing_service.return_book(&mut inventory, &mut user_manager, 1, 1).is_ok());
        assert!(inventory.get_book(1).unwrap().is_available);
        assert!(!user_manager.get_user(1).unwrap().borrowed_books.contains(&1));
    }

    #[test]
    fn test_borrow_unavailable_book () {
        let (mut inventory, mut user_manager, borrowing_service) = setup();

        assert!(borrowing_service.borrow_book(&mut inventory, &mut user_manager, 1, 1).is_ok());
        assert!(borrowing_service.borrow_book(&mut inventory, &mut user_manager, 1, 1).is_err());
    }
    #[test]
    fn test_return_book_not_borrowed_by_user() {
        let (mut inventory, mut user_manager, borrowing_service) = setup();

        // User 1 has not borrowed book 1
        let result = borrowing_service.return_book(&mut inventory, &mut user_manager, 1, 1);

        // Assuming user_manager.return_book returns an Err if the user hasn't borrowed the book
        assert!(result.is_err());
        // The book should still be available in the inventory as the return wasn't successful in user_manager
        assert!(inventory.get_book(1).unwrap().is_available);
    }
}
