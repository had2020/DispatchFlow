let lat = 0;
let lon = 0;

let other_user_points;

let map;
let userMarker;
let isMapInitialized = false;

function getLocation() {
  if (navigator.geolocation) {
    navigator.geolocation.watchPosition(success, error, {
      enableHighAccuracy: true,
      maximumAge: 0,
      timeout: 10000,
    });
  } else {
    alert("GPS is not supported, by this browser!");
  }
}

function success(position) {
  lat = position.coords.latitude;
  lon = position.coords.longitude;
  console.log("User Location: ", lat, lon);

  if (!isMapInitialized) {
    map = L.map("map", {
      zoomControl: true,
    }).setView([lat, lon], 17);

    L.tileLayer(
      "https://{s}.basemaps.cartocdn.com/dark_all/{z}/{x}/{y}{r}.png",
      {
        attribution: '&copy; <a href="https://carto.com/">CARTO</a>',
        maxZoom: 30,
      },
    ).addTo(map);

    isMapInitialized = true;
  }

  if (!userMarker) {
    userMarker = L.circle([lat, lon], {
      color: "red",
      fillColor: "#f03",
      fillOpacity: 0.3,
      radius: 10,
    }).addTo(map);
  } else {
    userMarker.setLatLng([lat, lon]);
  }

  //map.setView([lat, lon]);
}

function error(err) {
  console.error(`ERROR(${err.code}): ${err.message}`);
  alert("Failed to get your location.");
}

function addMarker(lat, lon, label = "Marker") {
  L.marker([lat, lon], {
    title: label,
  })
    .addTo(map)
    .bindPopup(label)
    .openPopup();
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

  fetch("https://76.167.195.153:8000/send_on_call", {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({
      user: localStorage.getItem("User"),
      team_name: localStorage.getItem("Team_name"),
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

setInterval(() => {
  fetch("https://76.167.195.153:8000/send_position", {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({
      user: localStorage.getItem("User"),
      team_name: localStorage.getItem("Team_name"),
      latitude: lat.toString(),
      longitude: lon.toString(),
    }),
  })
    .then((response) => response.text())
    .then((data) => {
      status_data = data;

      console.log(status_data);
    });

  fetch("https://76.167.195.153:8000/check_positions", {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({
      user: "",
      team_name: localStorage.getItem("Team_name"),
      latitude: "",
      longitude: "",
    }),
  })
    .then((response) => response.text())
    .then((data) => {
      status_data = data;

      let entries = status_data.split(",");

      let parsed = entries.map((entry) => {
        let parts = entry
          .trim()
          .split(":")
          .map((s) => s.trim());
        return {
          name: parts[0],
          lat: parseFloat(parts[1]),
          lon: parseFloat(parts[2]),
        };
      });

      for (let point of parsed) {
        if (point.name != localStorage.getItem("User")) {
          console.log(
            `${point.name} is at latitude ${point.lat} and longitude ${point.lon}`,
          );
          addMarker(point.lat, point.lon, point.name);
        }
      }

      console.log(status_data);
    });
}, 10000);
