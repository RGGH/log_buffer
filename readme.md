# Log Buffer (in Rust 🦀)

- uses double ended queue
- use to_owned() to avoid lifetime issues

---

Data Structure:

    VecDeque is used because it efficiently supports adding/removing elements from both ends.

Capacity Handling:

    When the buffer reaches its capacity, the oldest log is removed using pop_front.

Adding Logs:

    New logs are added to the end with push_back.

Retrieving Logs:

    get_logs returns a vector containing all the logs in order.
