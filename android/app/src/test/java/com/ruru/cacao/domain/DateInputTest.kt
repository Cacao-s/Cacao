package com.ruru.cacao.domain

import org.junit.Assert.assertEquals
import org.junit.Assert.assertNull
import org.junit.Test
import java.time.LocalDate

class DateInputTest {

    private val today = LocalDate.of(2026, 11, 13)
    private val expected = LocalDate.of(2023, 8, 30)

    @Test
    fun acceptsDigitsOnly() = assertEquals(expected, BirthdayInput.parse("20230830", today))

    @Test
    fun acceptsSeparators() {
        assertEquals(expected, BirthdayInput.parse("2023-08-30", today))
        assertEquals(expected, BirthdayInput.parse("2023/8/30", today))
        assertEquals(expected, BirthdayInput.parse(" 2023.08.30 ", today))
    }

    @Test
    fun rejectsInvalidAndFuture() {
        assertNull(BirthdayInput.parse("20230230", today))
        assertNull(BirthdayInput.parse("2023083", today))
        assertNull(BirthdayInput.parse("20271231", today))
        assertNull(BirthdayInput.parse("", today))
    }

    @Test
    fun dateInputAllowsFuture() = assertEquals(LocalDate.of(2027, 1, 5), DateInput.parse("20270105"))
}
