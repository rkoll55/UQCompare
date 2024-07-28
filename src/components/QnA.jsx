import React, { useState, useEffect } from "react";
import Styles from "./styles/review.module.css";

function QnA({ courseId }) {
  const [questions, setQuestions] = useState([]);
  const [answers, setAnswers] = useState({});
  const [newQuestion, setNewQuestion] = useState("");
  const [newAnswers, setNewAnswers] = useState({});
  const [message, setMessage] = useState("");

  useEffect(() => {
    fetchQuestions();
  }, [courseId]);

  const fetchQuestions = async () => {
    try {
      const response = await fetch(`${import.meta.env.VITE_API_URL}/api/getquestions/${courseId}`);
      if (!response.ok) {
        throw new Error("Network response was not ok");
      }
      const data = await response.json();
      setQuestions(data);

      // Fetch answers for each question
      data.forEach((question) => {
        const questionId = question.category.split("#")[1];
        fetchAnswers(courseId, questionId);
      });
    } catch (error) {
      console.error("Error fetching questions:", error);
    }
  }

  const fetchAnswers = async (courseId, questionId) => {
    try {
      const response = await fetch(`${import.meta.env.VITE_API_URL}/api/getanswers/${courseId}/${questionId}`);
      if (!response.ok) {
        throw new Error("Network response was not ok");
      }
      const data = await response.json();
      setAnswers((prevAnswers) => ({ ...prevAnswers, [questionId]: data }));
    } catch (error) {
      console.error("Error fetching answers:", error);
    }
  };

  const handleQuestionSubmit = async (event) => {
  event.preventDefault();
  const question = {
    course_id: courseId,
    text: newQuestion,
    date: new Date().toISOString(),
  };

  try {
    const response = await fetch(`${import.meta.env.VITE_API_URL}/api/createquestion`, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify(question),
    });

    const responseText = await response.text();

    if (response.ok) {
      setMessage("Question added successfully.");
      setNewQuestion("");
      fetchQuestions();
    } else {
      setMessage("Failed to add question.");
    }
  } catch (error) {
    console.error("Error adding question:", error);
    setMessage("An error occurred: " + error.message);
  }
};


  const handleAnswerSubmit = async (event, questionId) => {
    event.preventDefault();
    const answer = {
      course_id: courseId,
      question_id: questionId,
      text: newAnswers[questionId] || "",
      date: new Date().toISOString(),
    };

    try {
      const response = await fetch(`${import.meta.env.VITE_API_URL}/api/createanswer`, {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify(answer),
      });

      if (response.ok) {
        setMessage("Answer added successfully.");
        setNewAnswers((prev) => ({ ...prev, [questionId]: "" }));
        fetchAnswers(courseId, questionId);
      } else {
        setMessage("Failed to add answer.");
      }
    } catch (error) {
      setMessage("An error occurred: " + error.message);
    }
  };

  const handleAnswerChange = (questionId, value) => {
    setNewAnswers((prev) => ({ ...prev, [questionId]: value }));
  };

  return (
    <div className={Styles.qna}>
      <h1 style={{ color: "#44619e" }}>Questions and Answers</h1>
      {questions.length > 0 ? (
        questions.map((question) => {
          const questionId = question.category.split("#")[1];
          return (
            <div key={questionId} className={Styles.questionCard}>
              <p>{question.text}</p>
              {answers[questionId] && answers[questionId].length > 0 ? (
                answers[questionId].map((answer, index) => (
                  <div key={index} className={Styles.answerCard}>
                    <p>{answer.text}</p>
                  </div>
                ))
              ) : (
                <p>No answers available.</p>
              )}
              <form onSubmit={(e) => handleAnswerSubmit(e, questionId)}>
                <label>
                  Your answer:
                  <textarea
                    value={newAnswers[questionId] || ""}
                    onChange={(e) => handleAnswerChange(questionId, e.target.value)}
                    required
                  />
                </label>
                <button type="submit">Submit</button>
              </form>
            </div>
          );
        })
      ) : (
        <p>No questions available.</p>
      )}

      <form onSubmit={handleQuestionSubmit}>
        <div>
          <label>
            Your question:
            <textarea
              value={newQuestion}
              onChange={(e) => setNewQuestion(e.target.value)}
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

export default QnA;