// A much cleaner example of reference counting's use cases is presented here.
//
// Scenario: We have *Gadgets*, and many Gadgets can have a single *Owner* (I own a pencil, a
// screwdriver, a hammer etc). 
// Because of such a requirement, we want multiple Gadgets to point to the same Owner. This isn't
// easily possible without Rc<T>, because by default, we cannot share ownership of data in Rust (we
// can only borrow references). 

use std::rc::Rc;

#[derive(Debug)]
struct Owner {
	name: String,
}

#[derive(Debug)]
struct Gadget {
	id: i32,
	owner: Rc<Owner>,
}

fn main() {
	let gadget_owner: Rc<Owner> = Rc::new(
		Owner {
			name: "Gadget man".to_string(),
		}
	);

	// Create gadgets belonging to this single owner!
	// Cloning the `Rc<Owner>` value gives us a new pointer
	// to the same `Owner` value, incrementing the reference
	// count in the process.

	let g1 = Gadget {
		id: 1,
		owner: Rc::clone(&gadget_owner),
	};

	let g2 = Gadget {
		id: 2,
		owner: Rc::clone(&gadget_owner),
	};

	// Dispose of our local variable

	drop(gadget_owner);

	// despite dropping gadget_owner, we are still able to print out
	// the name of the `Owner` of the `Gadget`s. This is because we've only
	// dropped the single `Rc<Owner>`, now the `Owner` it points to.

	// As long as there are other `Rc<Owner>` values, it will remain allocated.

	println!("Gadget {} owned by {}", g1.id, g1.owner.name);
	println!("Gadget {} owned by {}", g2.id, g2.owner.name);

	// Caution: We can go from gadget -> owner via .owner.name, but if we for some reason
	// also want to go the reverse direction (from owner to his/her gadgets), that would be
	// very problematic:

	// An Rc pointer from Owner to Gadget will introduce a cycle between the values. This means
	// their reference count can never reach 0 and values will remain allocated forever: a memory leak.
	// In order to get around this, we can use `Weak` pointers.
}
