describe("home page for component library", () => {
  it("has links for the components that we have available to us", () => {
    cy
      .visit('/atoms')
      .get('[data-test="yew-router-link"]')
      .should("contain", 'Yew Router link to another page')
      .click()
      .url()
      .should("not.contain", "/atoms")
  });

  it("has a page title", () => {
    cy
      .visit("/atoms")
      .get('[data-test="page-title"]')
      .should('contain', 'Atom Components');
  });

  it('has a subtitle', () => {
    cy
      .visit('/atoms')
      .get('[data-test="subtitle"]')
      .should('contain', 'Links');
  })
})