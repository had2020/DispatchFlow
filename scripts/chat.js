if (localStorage.getItem("In_team") == "false") {
  window.location.href = "../user_pages/teams.html";
}

fetch("http://76.167.195.153:8000/check_chat", {
  method: "POST",
  headers: {
    "Content-Type": "application/json",
  },
  body: JSON.stringify({
    user: "",
    team_name: localStorage.getItem("Team_name"),
    message: "",
  }),
})
  .then((response) => response.text())
  .then((data) => {
    status_data = data;

    let chatContainer = document.querySelector(".chat-messages");

    status_data.split(",").forEach((entry) => {
      let parts = entry.trim().split(/\s+/);
      if (parts.length >= 2) {
        let username = parts[0];
        let message = parts.slice(1).join(" ");

        let messageDiv = document.createElement("div");

        messageDiv.className =
          username === localStorage.getItem("User")
            ? "message user"
            : "message other_user";
        messageDiv.textContent = `${username}: ${message}`;
        chatContainer.appendChild(messageDiv);
      }
    });

    console.log(status_data);
  });

document.getElementById("Reload").addEventListener("click", function () {
  window.location.reload();
});

document.getElementById("Send").addEventListener("click", function () {
  const username = localStorage.getItem("User");
  const team_name = localStorage.getItem("Team_name");
  let message = document.getElementById("Message_input").value;

  let status_data = "";

  fetch("https://76.167.195.153:8000/send_chat_message", {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({
      user: username,
      team_name: team_name,
      message: message,
    }),
  })
    .then((response) => response.text())
    .then((data) => {
      status_data = data;

      if (status_data == "sent") {
        window.location.reload();
      }
      console.log(status_data);
    });
});
