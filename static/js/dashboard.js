$("#btn-stockin").click(() => {
  $("#modal-stockin").modal("toggle")
})

$("#btn-stockout").click(() => {
  $("#modal-stockout").modal("toggle")
})

$("#modal-stockin-btn-add-field").click(() => {
  const n = $("#modal-stockin-body div.row").length;

  $("#modal-stockin-body").append(`
<div class="row no-gutters mt-3" id="modal-stockin-field${n + 1}">
  <div class="col">
    <label for="modal-stockin-input-field${n + 1}-key" class="sr-only">Key</label>
    <input class="form-control" id="modal-stockin-input-field${n + 1}-key" placeholder="key" required>
    <div class="my-2"></div>
    <label for="modal-stockin-input-field${n + 1}-value" class="sr-only">Value</label>
    <input class="form-control" id="modal-stockin-input-field${n + 1}-value" placeholder="value" required>
  </div>
  <div class="col-auto ml-2 d-flex align-items-stretch">
    <button class="btn btn-block btn-outline-danger" id="modal-stockin-btn-remove-field${n + 1}" aria-label="Remove this field" onclick="removeField('#modal-stockin-field${n + 1}')">&times;</button>
  </div>
</div>`)
})

const removeField = (f) => {
  $(f).detach();
}
