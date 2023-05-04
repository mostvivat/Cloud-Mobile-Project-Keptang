-- phpMyAdmin SQL Dump
-- version 5.2.1
-- https://www.phpmyadmin.net/
--
-- Host: 127.0.0.1
-- Generation Time: Apr 14, 2023 at 09:30 AM
-- Server version: 10.4.28-MariaDB
-- PHP Version: 8.2.4

SET SQL_MODE = "NO_AUTO_VALUE_ON_ZERO";
START TRANSACTION;
SET time_zone = "+00:00";


/*!40101 SET @OLD_CHARACTER_SET_CLIENT=@@CHARACTER_SET_CLIENT */;
/*!40101 SET @OLD_CHARACTER_SET_RESULTS=@@CHARACTER_SET_RESULTS */;
/*!40101 SET @OLD_COLLATION_CONNECTION=@@COLLATION_CONNECTION */;
/*!40101 SET NAMES utf8mb4 */;

--
-- Database: `keptang`
--

-- --------------------------------------------------------

--
-- Table structure for table `moneylist`
--

CREATE TABLE `moneylist` (
  `list_id` int(11) NOT NULL,
  `description` varchar(50) NOT NULL,
  `date` date NOT NULL,
  `amount` int(11) NOT NULL,
  `types` varchar(10) NOT NULL,
  `user_id` int(11) NOT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

--
-- Dumping data for table `moneylist`
--

INSERT INTO `moneylist` (`list_id`, `description`, `date`, `amount`, `types`, `user_id`) VALUES
(1, 'รุทให้', '2023-04-13', 120, 'income', 40954),
(39, 'พ่อให้', '2023-04-12', 500, 'income', 40956),
(40, 'กินข้าว', '2023-04-13', 50, 'expense', 40956),
(41, 'ขโมยเงิน', '2023-04-12', 5, 'income', 40956),
(42, 'ฟองขอยืม', '2023-04-14', 20, 'expense', 40956),
(43, 'แม่ให้', '2023-04-14', 100, 'income', 40956);

-- --------------------------------------------------------

--
-- Table structure for table `userdata`
--

CREATE TABLE `userdata` (
  `user_id` int(11) NOT NULL,
  `user_name` varchar(50) NOT NULL,
  `balance_total` int(11) NOT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

--
-- Dumping data for table `userdata`
--

INSERT INTO `userdata` (`user_id`, `user_name`, `balance_total`) VALUES
(40954, 'chawanwit', 120),
(40956, 'vivat', 535);

--
-- Indexes for dumped tables
--

--
-- Indexes for table `moneylist`
--
ALTER TABLE `moneylist`
  ADD PRIMARY KEY (`list_id`);

--
-- Indexes for table `userdata`
--
ALTER TABLE `userdata`
  ADD PRIMARY KEY (`user_id`);

--
-- AUTO_INCREMENT for dumped tables
--

--
-- AUTO_INCREMENT for table `moneylist`
--
ALTER TABLE `moneylist`
  MODIFY `list_id` int(11) NOT NULL AUTO_INCREMENT, AUTO_INCREMENT=44;
COMMIT;

/*!40101 SET CHARACTER_SET_CLIENT=@OLD_CHARACTER_SET_CLIENT */;
/*!40101 SET CHARACTER_SET_RESULTS=@OLD_CHARACTER_SET_RESULTS */;
/*!40101 SET COLLATION_CONNECTION=@OLD_COLLATION_CONNECTION */;
