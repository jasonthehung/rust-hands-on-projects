use serde;
use tokio::sync::{Semaphore, SemaphorePermit};

pub struct Museum {
    remaining_tickets: Semaphore,
}

#[derive(Debug)]
pub struct Ticket<'a> {
    permit: SemaphorePermit<'a>,
}

impl<'a> Ticket<'a> {
    pub fn new(permit: SemaphorePermit<'a>) -> Self {
        Self { permit }
    }
}

impl Drop for Ticket<'_> {
    fn drop(&mut self) {
        println!("Ticket dropped");
    }
}

impl Museum {
    pub fn new(totals: usize) -> Museum {
        Self {
            remaining_tickets: Semaphore::new(totals),
        }
    }

    pub fn get_tickets(&self) -> Option<Ticket<'_>> {
        match self.remaining_tickets.try_acquire() {
            Ok(permit) => Some(Ticket::new(permit)),
            Err(_) => None,
        }
    }

    pub fn ticket(&self) -> usize {
        self.remaining_tickets.available_permits()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        let museum = Museum::new(50); // @ total: 50
        let ticket = museum.get_tickets().unwrap(); // @ 50 - 1 = 49 remaining tickets

        assert_eq!(museum.ticket(), 49);

        let tickets: Vec<Ticket> = (0..49).map(|_| museum.get_tickets().unwrap()).collect(); // @ take all tickets

        assert_eq!(museum.ticket(), 0); // @ ramaining 0 tickets

        assert!(museum.get_tickets().is_none());

        drop(ticket);
        {
            let t = museum.get_tickets().unwrap();
            println!("Got ticket {:?}", t);
        }
        println!("I'm freeeeee");

        assert!(museum.get_tickets().is_some())
    }
}
