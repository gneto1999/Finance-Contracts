#![cfg_attr(not(feature = "std"), no_std, no_main)]


#[ink::contract]
mod expense_tracker {
    use ink::prelude::vec::Vec;
    use ink::prelude::string::String;
    use ink::storage::Mapping;
    use ink::storage::traits::StorageLayout;

    #[derive(Debug, PartialEq, Default)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std", derive(StorageLayout))]
    pub enum Category {
        Alimentacao,
        Transporte,
        Lazer,
        Saude,
        Educacao,
        Cobrancas,
        #[default]
        Outros,
    }

    #[derive(Debug, PartialEq)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std", derive(StorageLayout))]
    pub struct Expense {
        id: i64,
        title: String,
        description: String,
        amount: u64,
        date: String,
        category: Category,
    }

    #[ink(storage)]
    pub struct ExpenseTracker {
        expenses: Mapping<i64, Expense>,
        next_id: i64,
    }

    impl ExpenseTracker {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                expenses: Mapping::new(),
                next_id: 0,
            }
        }

        /// Cria uma nova despesa.
        #[ink(message)]
        pub fn create_expense(
            &mut self,
            title: String,
            description: String,
            amount: u64,
            date: String,
            category: Category,
        ) -> i64 {
            let id = self.next_id;
            let expense = Expense {
                id,
                title,
                description,
                amount,
                date,
                category,
            };
            self.expenses.insert(id, &expense);
            let next = self.next_id;
            self.next_id = next.checked_add(1).expect("Overflow ao incrementar next_file_id");
            id
        }

        /// Lê uma despesa pelo ID.
        #[ink(message)]
        pub fn read_expense(&self, id: i64) -> Option<Expense> {
            self.expenses.get(id)
        }

        /// Atualiza uma despesa existente.
        #[ink(message)]
        pub fn update_expense(
            &mut self,
            id: i64,
            title: String,
            description: String,
            amount: u64,
            date: String,
            category: Category,
        ) -> () {
            self.expenses.get(id)
                .map(|mut expense| {
                    expense.title = title;
                    expense.description = description;
                    expense.amount = amount;
                    expense.date = date;
                    expense.category = category;
                    self.expenses.insert(id, &expense);
                    true 
                })
                .unwrap_or(false);
        }

        /// Deleta uma despesa pelo ID.
        #[ink(message)]
        pub fn delete_expense(&mut self, id: i64) -> () {
            self.expenses.remove(id)
        }

        /// Lista todas as despesas armazenadas.
        #[ink(message)]
        pub fn list_expenses(&self) -> Vec<Expense> {
            let mut expenses = Vec::new();
            let mut current_id = 1;

            while current_id < self.next_id {
                if let Some(expense) = self.expenses.get(current_id) {
                    expenses.push(expense);
                }
                current_id = current_id.checked_add(1).expect("Overflow ao incrementar next_file_id");
            }

            expenses
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ink::env::test;

    #[test]
    fn test_crud_operations() {
        let mut contract = ExpenseTracker::new();

        let id1 = contract.create_expense(
            "Groceries".to_string(),
            "Bought fruits and vegetables".to_string(),
            50.0,
            "2025-01-01".to_string(),
            Category::Alimentacao,
        );

        let id2 = contract.create_expense(
            "Bus Ticket".to_string(),
            "Monthly pass".to_string(),
            100.0,
            "2025-01-02".to_string(),
            Category::Transporte,
        );

        assert!(contract.read_expense(id1).is_some());
        assert!(contract.read_expense(id2).is_some());

        contract.update_expense(
            id1,
            Some("Supermarket".to_string()),
            None,
            Some(60.0),
            None,
            None,
        );

        let updated_expense = contract.read_expense(id1).unwrap();
        assert_eq!(updated_expense.title, "Supermarket");
        assert_eq!(updated_expense.amount, 60.0);

        assert!(contract.delete_expense(id2));
        assert!(contract.read_expense(id2).is_none());
    }
}
