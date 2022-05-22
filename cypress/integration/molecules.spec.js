describe('small groups of atoms together', () => {
  it('molecules page', () => {
    cy
      .visit('/molecules')
      .get('[data-test="page-title"]')
      .should('contain', 'Molecule Components');
  });

  // it('navbar', () => {
  //   cy
  //     .visit('/molecules')
  //     .get('[data-test="navbar-title"]')
  //     .should('contain', 'Navbar')
  //     .get('[data-test="navbar-home"]')
  //     .click()
  //     .url()
  //     .should('not.contain', '/molecules')
  // })
})