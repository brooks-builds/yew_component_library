describe('small groups of atoms together', () => {
  it('molecules page', () => {
    cy
      .visit('/molecules')
      .get('[data-test="page-title"]')
      .should('contain', 'Molecule Components')
      .get('[data-test="navbar-title"]')
      .should('contain', 'Navbar')
  });

  it('navbar', () => {
    cy
      .visit('/molecules')
      .get('[data-test="navbar-icon"]')
      .click()
      .url()
      .should('not.contain', '/molecules')
  })
})