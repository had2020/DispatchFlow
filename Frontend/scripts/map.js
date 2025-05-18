let user_latitude = 0;
let user_longitude = 0;

function getLocation() {
  if (navigator.geolocation) {
    navigator.geolocation.getCurrentPosition(success, error);
  } else {
    alert("Geolocation is not supported by this browser.");
  }
}

function success(position) {
  console.log(position.coords.latitude + ", " + position.coords.longitude);
  user_latitude = position.coords.latitude;
  user_longitude = position.coords.longitude;

  let map = L.map("map").setView([user_latitude, user_longitude], 17); // third parameter is zoom
  L.tileLayer("https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png", {
    maxZoom: 19,
  }).addTo(map);
  var marker = L.marker([user_latitude, user_longitude]).addTo(map);
}

function error() {
  alert("Sorry, no position available.");
}

getLocation();

if (localStorage.getItem("Duty") == "true") {
  document.getElementById("duty_button").innerHTML =
    "⚫️ Set to Off-Call Status";
}

// report for duty
document.getElementById("duty_button").addEventListener("click", function () {
  let new_status = "";
  if (localStorage.getItem("Duty") == "true") {
    new_status = "false";
  } else {
    new_status = "true";
  }

  fetch("http://localhost:8000/send_on_call", {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({
      user: localStorage.getItem("User"),
      team_name: localStorage.getItem("Pass"),
      status: new_status,
    }),
  })
    .then((response) => response.text())
    .then((data) => {
      status_data = data;

      if (status_data == "good") {
        if (localStorage.getItem("Duty") == "true") {
          localStorage.setItem("Duty", "false");
          document.getElementById("duty_button").innerHTML =
            "🟢 Set to On-Call Status";
        } else {
          localStorage.setItem("Duty", "true");
          document.getElementById("duty_button").innerHTML =
            "⚫️ Set to Off-Call Status";
        }
      }

      console.log(status_data);
    });
});
