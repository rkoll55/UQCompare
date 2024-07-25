import React, { useState, useEffect} from "react";
import Styles from "./styles/review.module.css";

function Review({courseId}) {
  const [rating, setRating] = useState(0);
  const [text, setText] = useState("");
  const [message, setMessage] = useState("");
  const [reviews, setReviews] = useState([]);

  useEffect(() => {
    fetch("http://127.0.0.1:8000/api/getreviews/" + courseId) // Replace COURSE_CODE with the actual course code or make it dynamic
      .then((response) => response.json())
      .then((data) => setReviews(data))
      .catch((error) => console.error("Error fetching reviews:", error));
  }, []);

  const handleSubmit = async (event) => {
    event.preventDefault();

    const review = {
      course_id: courseId,
      rating: parseInt(rating, 10),
      text: text,
    };

    try {
      const response = await fetch("http://127.0.0.1:8000/api/createreview", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify(review),
      });

      if (response.ok) {
        setMessage("Review added successfully.");
        setRating(0);
        setText("");
        fetch("http://127.0.0.1:8000/reviews/COURSE_CODE")
          .then((response) => response.json())
          .then((data) => setReviews(data))
          .catch((error) => console.error("Error fetching reviews:", error));
      } else {
        setMessage("Failed to add review.");
      }
    } catch (error) {
      setMessage("An error occurred: " + error.message);
    }
  };

  return (
    <div className={Styles.review}>
      <h1 style={{ color: "#44619e" }}>Reviews</h1>
      {reviews.length > 0 ? (
        reviews.map((review, index) => (
          <div key={index} className={Styles.reviewCard}>
            <p>Rating: {review.rating}</p>
            <p>{review.text}</p>
          </div>
        ))
      ) : (
        <p>No reviews available.</p>
      )}

      <form onSubmit={handleSubmit}>
        <div>
          <label>
            Leave a review:
            <textarea
              value={text}
              onChange={(e) => setText(e.target.value)}
              required
            />
          </label>
        </div>
        <div>
          <label>
            Rating:
            <input
              type="number"
              value={rating}
              onChange={(e) => setRating(e.target.value)}
              min="1"
              max="10"
              required
            />
          </label>
        </div>
        <button type="submit">Submit</button>
      </form>
      {message && <p>{message}</p>}
    </div>
  );
}

export default Review;
