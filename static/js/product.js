$("#modal-remove-btn-remove").click(() => {
  $.ajax({
    method: "DELETE",
    url: "",
  }).done(() => {
    window.location.href = "/dashboard"
  })
})
