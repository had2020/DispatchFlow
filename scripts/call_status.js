let status_data = "";

document.getElementById("Reload").addEventListener("click", function () {
  window.location.reload();
});

fetch("https://76.167.195.153:8000/check_all_status", {
  method: "POST",
  headers: {
    "Content-Type": "application/json",
  },
  body: JSON.stringify({
    user: localStorage.getItem("User"),
    team_name: localStorage.getItem("Team_name"),
    status: "lol",
  }),
})
  .then((response) => response.text())
  .then((data) => {
    status_data = data;

    let dutyList = document.getElementById("dutyList");

    status_data.split(",").forEach((entry) => {
      let match = entry.trim().match(/^(\w+)\s+\((On Duty|Off Duty)\)$/i);
      if (match) {
        let username = match[1];
        let status = match[2];
        let icon = status === "On Duty" ? "🟢" : "🔴";

        let li = document.createElement("li");
        li.textContent = `${username} | ${icon} ${status}`;
        dutyList.appendChild(li);
      }
    });

    console.log("Returned: " + status_data);
  });
